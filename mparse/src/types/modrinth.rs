use std::path::Path;

use super::ModpackProviderMetadata;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpack {
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<ModrinthModpackFiles>,
}

impl ModpackProviderMetadata for ModrinthModpack {
    fn overrides_dir(&self) -> &str {
        "overrides"
    }

    fn modpack_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpackFiles {
    pub path: Box<Path>,
    pub hashes: ModpackFileHashes,
    pub env: ModpackEnv,
    pub downloads: Vec<String>,
    pub file_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct ModpackFileHashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Debug, Deserialize)]
pub struct ModpackEnv {
    pub server: String,
    pub client: String,
}
