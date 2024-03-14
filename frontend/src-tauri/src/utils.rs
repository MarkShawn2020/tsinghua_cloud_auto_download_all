use std::fs::create_dir_all;
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use futures_util::stream::TryStreamExt;
use serde_json::{from_str, json};
use tauri::{AppHandle, Manager, State};
use tokio::fs::File;
use tokio::io;
use tokio_util::io::StreamReader;

use crate::schema::{DirList, ListData, TheError};

// Import the TryStreamExt trait


pub async fn fetch_dir_list(path: &String) -> Result<DirList, TheError> {
    println!("-- listing {}", path);

    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/689824200edb49888695/dirents?path={}", path))
        .await?
        .text()
        .await?;

    Ok(from_str::<ListData>(&response)?.dirent_list)
}

pub async fn fetch_file_content(root_path: &Path, path: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("-- downloading {}", path);

    create_dir_all(root_path)?;

    let file_path = root_path.join(path);

    let response = reqwest::get(path).await?;

    let mut file = std::fs::File::create(&file_path)?;

    let mut content = Cursor::new(response.bytes().await?);

    std::io::copy(&mut content, &mut file)?;

    println!("File has been downloaded and saved to {:?}", file_path);

    Ok(())
}

pub async fn recursive_fetch_and_emit(app: &AppHandle, store_path: &Path, root_path: &String, stop_signal: State<'_, Arc<AtomicBool>>)  -> Result<(), String>{
    let mut paths = vec![root_path.to_owned()];

    while let Some(path) = paths.pop() {

        // how to: if received STOP signal from another invoke, then break here
        if stop_signal.load(Ordering::SeqCst) {
            println!("Stopped fetching data since interrupted");
            break;
        }

        match fetch_dir_list(&path).await {
            Ok(data) => {
                // println!("emitting: {}", data);
                let _ = app.emit_all("list_data", json!({"children": &data, "parent": &path}));

                for item in &data {
                    if let Some(fp) = &item.folder_path {
                        paths.push(fp.clone());
                    } else if let Some(fp) = &item.file_path {
                        fetch_file_content(&store_path, fp); // do not await
                            // .await.map_err(|e| e.to_string())?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching data: {}", e)
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
