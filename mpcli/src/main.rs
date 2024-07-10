use clap::{Parser, Subcommand};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use dirs::config_dir;

#[cfg(unix)]
use std::env::var;

use log::{error, info, warn};
use mparse::{
    get_modpack_manifest, ForgeModpack, ModpackMetadata, ModpackProvider, ModrinthModpack,
};
use serde_json::from_str;
use std::{ffi::OsString, path::PathBuf};

mod install;
mod logger;

#[cfg(unix)]
const DEFAULT_MINECRAFT_HOME: &str = "~/.minecraft";
#[cfg(target_os = "windows")]
const DEFAULT_MINECRAFT_HOME: &str = "%APPDATA%\\.minecraft";
#[cfg(target_os = "macos")]
const DEFAULT_MINECRAFT_HOME: &str = "~/Library/Application Support/minecraft";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
    /// Path to .minecraft dir where the game files are stored
    #[arg(short, long, default_value=get_default_minecraft_home())]
    minecraft_home: PathBuf,

    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install a modpack
    Install(InstallArgs),
    /// Uninstall a modpack
    Uninstall(UninstallArgs),
    /// Show modpack information
    Info(InfoArgs),
}

#[derive(Debug, Parser)]
struct InstallArgs {
    /// Modpack name. Defaults to the filename of the modpack
    #[arg(short, long)]
    name: Option<String>,

    /// File to install. Must be a ZIP archive
    file: PathBuf,
}

#[derive(Debug, Parser)]
struct UninstallArgs {
    /// Modpack name. Defaults to the filename of the modpack
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Debug, Parser)]
struct InfoArgs {
    /// File to install. Must be a ZIP archive
    file: PathBuf,
}

fn get_default_minecraft_home() -> OsString {
    OsString::from(DEFAULT_MINECRAFT_HOME)
}

#[derive(Debug)]
enum ManifestType {
    Forge(ForgeModpack),
    Modrinth(ModrinthModpack),
}

fn show_modpack_info(meta: ModpackMetadata) {
    let meta = match meta.loader {
        ModpackProvider::Forge => {
            let file = from_str::<ForgeModpack>(&meta.raw);

            if file.is_err() {
                error!(
                    "Unable to parse forge modpack information: {}",
                    file.unwrap_err()
                );
                return;
            }

            ManifestType::Forge(file.unwrap())
        }
        ModpackProvider::Modrinth => {
            let file = from_str::<ModrinthModpack>(&meta.raw);

            if file.is_err() {
                error!(
                    "Unable to parse modrinth modpack information: {}",
                    file.unwrap_err()
                );
                return;
            }

            ManifestType::Modrinth(file.unwrap())
        }
        ModpackProvider::None => {
            panic!("somehow get_modpack_manifest provided a 'None' value, which shouldn't have happened");
        }
    };

    dbg!(meta);
}

fn subcmd_info(args: InfoArgs) {
    info!("Showing info for pack {}", args.file.display());
    match get_modpack_manifest(args.file) {
        Err(err) => error!("Unable to unpack modpack: {}", err),
        Ok(modpack) => show_modpack_info(modpack),
    };
}

fn subcmd_install(args: InstallArgs, install_dir: PathBuf) {
    // resolve `install_dir` by OS
    let mut install_dir = if install_dir.as_os_str() == get_default_minecraft_home() {
        error!("SAME LMAO!");

        #[cfg(any(target_os = "windows", target_os = "macos"))]
        let new_install_dir = config_dir().unwrap().as_path().join(".minecraft");

        #[cfg(unix)]
        let new_install_dir = std::path::absolute(install_dir).unwrap();

        new_install_dir
    } else if !install_dir.ends_with(".minecraft") {
        let mut install_dir = install_dir.to_path_buf();
        install_dir.push(".minecraft");

        std::path::absolute(install_dir).unwrap()
    } else {
        std::path::absolute(install_dir).unwrap()
    };

    // transform `install_dir` as required
    install_dir.push(format!(
        "version/{}",
        args.file
            .with_extension("")
            .file_name()
            .unwrap()
            .to_string_lossy()
    ));

    // finalize `install_dir`
    let install_dir = std::path::absolute(install_dir).unwrap();
    info!(
        "Installing pack {} to {}",
        args.file.display(),
        install_dir.display()
    );
}
fn subcmd_uninstall(args: UninstallArgs, install_dir: PathBuf) {
    todo!()
}

fn main() {
    let global_args = Args::parse();

    let _ = logger::setup_logger(global_args.verbosity.log_level_filter());
    warn!("This program is partially complete, running in 'dry run' mode");

    match global_args.subcommand {
        Commands::Info(args) => subcmd_info(args),
        Commands::Install(args) => subcmd_install(args, global_args.minecraft_home),
        Commands::Uninstall(args) => subcmd_uninstall(args, global_args.minecraft_home),
    }
}