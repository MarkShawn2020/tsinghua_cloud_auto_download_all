use std::io::Cursor;
use std::path::Path;

use serde_json::{from_str, json};
use tauri::{AppHandle, Manager};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast;
use std::ffi::c_int;
use std::fs::create_dir_all;
use std::time::Duration;

use crate::schema::{DirList, ListData, TheError};

pub async fn download_file(store_path: String, repo: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("-- downloading {:?}", path);

    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/d/{}/files/?p={}&dl=1", repo, path)).await?;

    let mut content = Cursor::new(response.bytes().await?);

    let local_path = Path::new(&store_path).join(path.clone().strip_prefix("/").unwrap());

    println!("-- local path: {:?}", local_path);

    let mut file = std::fs::File::create(local_path)?;

    std::io::copy(&mut content, &mut file)?;

    println!("File has been downloaded and saved to {:?}", path);

    Ok(())
}

pub async fn fetch_dir_list(repo: String, path: String) -> Result<DirList, TheError> {
    println!("-- listing {}", path);

    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/{}/dirents?path={}", repo, path)).await?.text().await?;

    Ok(from_str::<ListData>(&response)?.dirent_list)
}


pub async fn producer(app: AppHandle, store_path: String, repo: String, root_path: String, stop_signal: Arc<AtomicBool>, tx: broadcast::Sender<(String, c_int)>) {
    let mut paths = vec![root_path.clone()];
    let mut index = 0;

    while let Some(path) = paths.pop() {
        if stop_signal.load(Ordering::SeqCst) {
            println!("stopped since interrupted");
            break;
        }

        create_dir_all(Path::new(&store_path.clone()).join(path.clone().strip_prefix("/").unwrap())).unwrap();

        match fetch_dir_list(repo.clone(), path.clone()).await {
            Ok(data) => {
                // println!("emitting: {}", data);
                let _ = app.emit_all("core", json!({"children": &data, "parent": &path, "type": "list-dirs"}));

                for item in &data {
                    if let Some(fp) = &item.folder_path {
                        paths.push(fp.clone());
                    } else if let Some(fp) = &item.file_path {
                        tx.send((fp.clone(), index)).expect("TODO: panic message");
                        index += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching data: {}", e)
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

pub async fn consumer(store_path: String, repo: String, path: String) {
    download_file(store_path.clone(), repo.clone(), path.clone()).await.expect("TODO: panic message");
}
