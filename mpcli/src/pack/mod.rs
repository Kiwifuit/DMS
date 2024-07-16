use std::fs::{copy, create_dir_all, read_dir, File};
use std::io::prelude::*;

use model::GenericModpackManifest;
mod model;

use dialoguer::{theme::ColorfulTheme as Theme, FuzzySelect};
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info};
use serde_json::to_writer;
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use thiserror::Error;
use zip::{
    write::{FileOptions, ZipWriter},
    CompressionMethod,
};

use crate::cmd::info;

pub const MANIFEST_NAME: &str = "mpack-mod.json";

#[derive(Debug, Error)]
pub enum PackError {
    #[error("error while initiating progbar: {0}\n\tNote that under normal circumstances, this message should not be visible")]
    ProgbarTemplate(#[from] indicatif::style::TemplateError),

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("zip error: {0}")]
    Compression(#[from] zip::result::ZipError),

    #[error("required folder does not exist: {0}")]
    DoesNotExist(String),

    #[error("{0} (code 1)")] // TODO: Devise a scheme for these dev codes
    StripPrefix(#[from] std::path::StripPrefixError),

    #[error("no modpacks available/no such modpack exists")]
    NoModpack,
}

pub fn select_modpack(
    args: crate::types::ExportArgs,
    modpacks: &Vec<PathBuf>,
) -> Result<usize, PackError> {
    if args.name.is_none() {
        Ok(FuzzySelect::with_theme(&Theme::default())
            .with_prompt("Select modpack to export:")
            .items(
                &modpacks
                    .iter()
                    .map(|a| a.file_name().unwrap().to_str().unwrap())
                    .collect::<Vec<&str>>(),
            )
            .interact()
            .unwrap())
    } else {
        let modpack_name = &args.name.clone().unwrap();
        let selected = modpacks
            .iter()
            .enumerate()
            .filter_map(|(index, path)| {
                if path.ends_with(modpack_name) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        if let Some(index) = selected.first() {
            Ok(index.to_owned())
        } else {
            Err(PackError::NoModpack)
        }
    }
}

pub fn list_modpacks<P: AsRef<Path>>(home_dir: P) -> Result<Vec<PathBuf>, PackError> {
    info!("Searching modpacks");

    let modpacks = read_dir(home_dir)?
        .into_iter()
        .filter_map(|a| {
            if !a.as_ref().unwrap().path().is_dir() {
                return None;
            }
            Some(a.unwrap().path())
        })
        .collect::<Vec<PathBuf>>();

    if modpacks.is_empty() {
        error!("No modpacks to select!");
        return Err(PackError::NoModpack);
    }

    info!("{} modpack(s) to select", modpacks.len());
    Ok(modpacks)
}

pub fn write_modpack<P>(modpack_path: &P, archive_dir: &P) -> Result<(), PackError>
where
    P: AsRef<Path>,
{
    debug!(
        "Packing modpack at {}",
        modpack_path.as_ref().file_name().unwrap().to_string_lossy()
    );

    let modpack_name = modpack_path.as_ref().file_name().unwrap();
    let modpack_file = File::create_new(
        archive_dir
            .as_ref()
            .join(modpack_name)
            .with_extension("zip"),
    )?;
    let mut archive = ZipWriter::new(modpack_file);
    let mut manifest = model::GenericModpackManifest::default();

    info!(
        "Creating modpack archive {:?} in {:?}",
        archive_dir
            .as_ref()
            .join(modpack_name)
            .with_extension("zip"),
        archive_dir.as_ref().to_string_lossy()
    );
    manifest.name = modpack_name.to_string_lossy();
    manifest.base_dir = modpack_path.as_ref().to_path_buf();

    if !modpack_path.as_ref().join("mods").exists() {
        return Err(PackError::DoesNotExist("mods".to_string()));
    }

    println!("Indexing modpack folders");
    for folder in vec!["mods", "config", "resourcepacks"] {
        let base_dir = modpack_path.as_ref().join(&folder);
        let mut file_pool = vec![];
        if let Err(e) = find_files(&base_dir, &mut file_pool) {
            error!("Failed to find files within {:?}: {}", folder, e);
        }

        info!("Found {} file(s) in {} folder...", file_pool.len(), folder);
        let progress = ProgressBar::new(file_pool.len() as u64)
            .with_message(folder)
            .with_style(
                ProgressStyle::with_template("{msg:>15} [{wide_bar}] {percent}%")?
                    .progress_chars("=> "),
            );

        file_pool
            .iter()
            .map(|file| {
                manifest.register_file(file, folder)?;

                progress.inc(1);
                Ok(())
            })
            .collect::<Result<Vec<()>, std::io::Error>>()?;
        progress.finish();
    }

    let tempdir = TempDir::new("modpack")?;
    let zipfs = make_zipfs_structure(&tempdir, &manifest);
    if zipfs.is_err() {
        archive.finish()?;
        tempdir.close()?;
        return Err(PackError::Io(zipfs.unwrap_err()));
    }

    let zip_res = zip_dir(&mut archive, &zipfs.unwrap(), &tempdir.path().to_path_buf());

    if zip_res.is_err() {
        archive.finish()?;
        tempdir.close()?;
        return Err(zip_res.unwrap_err());
    }

    info!("Exported modpack to {:?}", modpack_name.to_string_lossy());
    println!("Exported modpack to {:?}", modpack_name.to_string_lossy());
    Ok(())
}

fn zip_dir<F, P>(archive: &mut ZipWriter<F>, path: &P, base_dir: &PathBuf) -> Result<(), PackError>
where
    F: Write + Seek,
    P: AsRef<Path>,
{
    let options = FileOptions::default()
        .unix_permissions(0o644)
        .compression_method(CompressionMethod::Bzip2)
        .compression_level(Some(9));
    let mut buf = vec![];
    let mut files = vec![];
    find_files_and_dirs(path, &mut files)?;

    info!("Zipping dir {} to modpack", path.as_ref().display());
    let progress = ProgressBar::new(files.len() as u64)
        .with_message("Zipping files")
        .with_style(
            ProgressStyle::with_template("{msg:>15} [{wide_bar}] {percent}%")?
                .progress_chars("=> "),
        );

    for path in files {
        let arc_path = path
            .strip_prefix(&base_dir)?
            .to_str()
            .map(str::to_owned)
            .unwrap()
            .replace("\\", "/");

        if path.is_dir() {
            archive.add_directory(&arc_path, options)?;
            debug!("Created dir {}", arc_path);
        } else {
            archive.start_file(&arc_path, options)?;
            debug!("Created file {}", arc_path);

            let mut file = File::open(path)?;

            let written = file.read_to_end(&mut buf)?;
            archive.write_all(&buf)?;
            buf.clear();

            debug!("Written {} bytes to archive", written);
        }

        progress.inc(1);
    }
    progress.finish();

    Ok(())
}

fn make_zipfs_structure(
    dir: &TempDir,
    manifest: &GenericModpackManifest,
) -> std::io::Result<PathBuf> {
    let base_dir = &manifest.base_dir;

    info!(
        "Moving files to temporary directory: {}",
        dir.path().display()
    );
    for file in &manifest.files {
        let real_path = base_dir.join(&file.path);
        let dest_path = dir.path().join(&file.path);

        debug!("Copying {} -> {}", real_path.display(), dest_path.display());

        if !dest_path.parent().unwrap().exists() {
            debug!(
                "Created parent dir {}",
                dest_path.parent().unwrap().display()
            );
            create_dir_all(dest_path.parent().unwrap())?;
        }

        // Create file
        File::create_new(&dest_path)?;
        copy(&real_path, &dest_path)?;
        debug!("Copied {} -> {}", real_path.display(), dest_path.display());
    }

    info!("Dumping manifest");
    let mut manifest_file = File::create_new(dir.path().join(MANIFEST_NAME))?;
    to_writer(&mut manifest_file, manifest)?;

    info!("Copied all files, ready for zipping");
    Ok(dir.path().to_path_buf())
}

fn find_files<P: AsRef<Path>>(path: &P, pool: &mut Vec<PathBuf>) -> Result<(), PackError> {
    debug!("Searching files in folder: {}", path.as_ref().display());

    for entry in read_dir(path)? {
        let path = entry?.path();

        if path.is_dir() {
            find_files(&path, pool)?;
        } else {
            debug!("Found file: {}", path.display());
            pool.push(path);
        }
    }

    Ok(())
}

fn find_files_and_dirs<P: AsRef<Path>>(path: &P, pool: &mut Vec<PathBuf>) -> Result<(), PackError> {
    info!("Indexing zipfs");

    for entry in read_dir(path)? {
        let path = entry?.path();

        if path.is_dir() {
            find_files(&path, pool)?;
        }

        debug!("Found entry: {}", path.display());
        pool.push(path);
    }

    Ok(())
}