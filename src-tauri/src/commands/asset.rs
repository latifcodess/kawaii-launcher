use serde::{Deserialize, Serialize};
use crate::commands::version::Version;
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Assets {
    pub objects: HashMap<String, Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub hash: String,
    pub size: u32,
}

pub async fn get_assets(version: Version) -> Assets {
    let assets_indexes_path = Path::new("minecraft/assets/indexes");
    let path = assets_indexes_path.join(format!("{}.json", version.asset_index.id));
    if path.exists() {
        // get the deserialized data form piston-meta
        let file: File = File::open(path).expect("Failed to open path");
        let assets: Assets = serde_json::from_reader(file).expect("Failed to deserialized");

        // return the deserialized data
        return assets;
    }

    // get the deserialized data form piston-meta
    let assets = reqwest::get(version.asset_index.url)
        .await
        .expect("Failed to get version manifest")
        .json::<Assets>()
        .await
        .expect("Failed to parse version manifest");

    // return the deserialized data
    return assets;
}