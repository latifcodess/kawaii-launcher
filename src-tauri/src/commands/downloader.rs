use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::commands::version::{Version, get_versions};

async fn download_library(version: Version) -> tauri::async_runtime::JoinHandle<()>  {
    // new thread with a handle to wait on
    tauri::async_runtime::spawn(async move {

        // library path
        let library_folder = Path::new("minecraft/libraries");

        // iter on all libraries to download
        for lib in version.libraries {
            // list of artifacts to download
            let mut artifacts = Vec::new();

            // if there is an artifact we push it in the list
            if let Some(ref art) = lib.downloads.artifact {
                artifacts.push(art);
            }

            // if there is classifiers we push them in the list
            if let Some(ref classifiers) = lib.downloads.classifiers {
                for native_artifact in classifiers.values() {
                    artifacts.push(native_artifact);
                }
            }

            // iter on all artifacts
            for artifact in artifacts {

                // path to download the artifact
                let path = library_folder.join(&artifact.path);

                // create all dir to the path
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(parent).expect("Failed to create parent directory");
                    }
                }

                // if the path doesn't already exist
                if !path.as_path().exists() {
                    println!("Downloading {}", artifact.url);

                    // request the bytes
                    let req = reqwest::get(&artifact.url)
                        .await
                        .expect("Failed to download library")
                        .bytes()
                        .await
                        .expect("Failed to download binary");

                    // create the file
                    let mut file = File::create(path).expect("Failed to create file");

                    // write the bytes into the file
                    file.write_all(&req).expect("Failed to write to file");
                }
            }
        }
    })
}

async fn download_version(version: Version) -> tauri::async_runtime::JoinHandle<()> {
    // new thread with a handle to wait on
    tauri::async_runtime::spawn(async move {
        let version_folder = Path::new("minecraft/versions");
        let path  = version_folder.join(format!("{}/{}.jar", version.id, version.id));
        let json_path  = version_folder.join(format!("{}/{}.json", version.id, version.id));

         if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create parent directory");
            }
        }

         if let Some(parent) = json_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create parent directory");
            }
        }

        if !path.as_path().exists() {
            println!("Downloading {}", version.id);

            // request the bytes
            let req = reqwest::get(&version.downloads.get("client").unwrap().url)
                .await
                .expect("Failed to download library")
                .bytes()
                .await
                .expect("Failed to download binary");

            // create the file
            let mut file = File::create(path).expect("Failed to create file");

            // write the bytes into the file
            file.write_all(&req).expect("Failed to write to file");
        }

        if !json_path.as_path().exists() {
         println!("Downloading {}", version.id);

            let versions = get_versions().await;
            let selected_version = versions.versions.into_iter().find(|v| v.id == version.id).expect("Failed to find version in manifest");

            let json_version = reqwest::get(selected_version.url)
            .await
            .expect("Failed to get version manifest")
            .bytes()
            .await
            .expect("Failed to parse version manifest");

            // create the file
            let mut file = File::create(json_path).expect("Failed to create file");

            // write the bytes into the file
            file.write_all(&json_version).expect("Failed to write to file");   
        }
    })
}

pub async fn start_download(version: Version) {
    // get the handle of the thread
    let library_handle = download_library(version.clone()).await;

    let version_handle = download_version(version.clone()).await;

    // wait on the thread
    library_handle.await.expect("Failed to join library handle");
    version_handle.await.expect("Failed to join version handle");
}