use std::fs;
use std::path::Path;
use std::process::Command;
use std::fs::canonicalize;

use crate::commands::{downloader, version};
use crate::commands::version::{Library, get_version};

fn create_folders() {
    let folders = [
        "minecraft",
        "minecraft/assets",
        "minecraft/libraries",
        "minecraft/versions",
    ];
    for folder in folders
    {
        if !Path::new(folder).exists() {
            fs::create_dir(folder).expect("Could not create folder");
        }
    }
}

fn get_libraries(dir: &Path) -> Vec<String> {
    let mut libraries = Vec::new();
    let dir = fs::read_dir(dir).expect("Failed to read dir");
    for file in dir {
        let path  = file.unwrap().path();
        if path.is_dir() {
            libraries.extend(get_libraries(&path));
        } else if path.extension().unwrap() == "jar" {
            let absolute = canonicalize(&path).unwrap();
            libraries.push(absolute.to_string_lossy().to_string());
        }

    }
    libraries
}

#[tauri::command]
pub async fn launch_game(username: String, version: String) {
    create_folders();
    downloader::start_download(get_version(version.clone()).await).await;
    println!("Start launching the game as {} in {}!", username, version.clone());
    let base_folder = "minecraft/";
    let library_folder = "minecraft/libraries";
    let assets_folder = "minecraft/assets";
    let version_folder = "minecraft/versions";
    let version = get_version(version).await;


    Command::new("java")
    .args(&[
        "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump", 
        "-Xss1M",
        &format!("-Djava.library.path={}", library_folder),
        &format!("-Djna.tmpdir={}", library_folder),
        &format!("-Dorg.lwjgl.system.SharedLibraryExtractPath={}", library_folder),
        &format!("-Dio.netty.native.workdir={}", library_folder),
        &format!("-Dminecraft.launcher.brand={}", "Kawaii"),
        &format!("-Dminecraft.launcher.version={}", 100),
        "-cp",
        format!("{}/{}/{}.jar:{}", version_folder, &version.id, &version.id, get_libraries(Path::new(library_folder)).join(":")).as_str(),
        &version.main_class, 

        "--username",
        "player", 
        "--version",
        &version.id,     
        "--gameDir",
        base_folder,   
        "--assetsDir",
        assets_folder,      
        "--assetIndex",
        &version.asset_index.id,
        "--uuid",
        "0",        
        "--accessToken",
        "0",     
        "--versionType",
        &version.r#type,
    ])
    .spawn()
    .expect("Failed to run the game");
}