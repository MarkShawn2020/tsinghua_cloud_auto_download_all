use std::fs::create_dir_all;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
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


pub async fn recursive_fetch_and_emit(app: &AppHandle, store_path: &Path, root_path: &String, stop_signal: State<'_, Arc<AtomicBool>>)  {

    let (put, get) = mpsc::channel();

    let producer_handle = thread::spawn(async move || {
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
                            put.send(fp.clone()).unwrap();
                            tokio::time::sleep(Duration::from_secs(0)).await;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching data: {}", e)
                }
            }

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    use std::io::{self, ErrorKind};

    // Assuming you have a custom error type or you can use a generic error type for simplicity.
// Here, we use Box<dyn std::error::Error> for demonstration.
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    let downloader_handle = thread::spawn(async move || -> Result<()> {
        for path in get {
            println!("-- downloading {:?}", &path);

            create_dir_all(root_path)?;

            let response = reqwest::get(&path).await?;

            let mut content = Cursor::new(response.bytes().await?);

            let mut file = std::fs::File::create(store_path.join(&path))?;

            std::io::copy(&mut content, &mut file)?;

            println!("File has been downloaded and saved to {:?}", &path);
        }

        Ok(())

    });

    // Wait for both threads to complete
    producer_handle.join().unwrap();
    downloader_handle.join().unwrap();

    println!("Both processes have finished.");
}
