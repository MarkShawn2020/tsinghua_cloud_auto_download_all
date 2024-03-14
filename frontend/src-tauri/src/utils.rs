use serde_json::{from_str, json};
use tauri::{AppHandle, Manager, State};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crate::schema::{DirList, ListData, TheError};

pub async fn fetch_data(path: &str) -> Result<DirList, TheError> {
    println!("fetching data...");

    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/689824200edb49888695/dirents?path={}", path))
        .await?
        .text()
        .await?;

    Ok(from_str::<ListData>(&response)?.dirent_list)
}

pub async fn recursive_fetch_and_emit(app: &AppHandle, root_path: &str, stop_signal: State<'_, Arc<AtomicBool>>) {
    let mut paths = vec![root_path.to_owned()];

    while let Some(path) = paths.pop() {

        // how to: if received STOP signal from another invoke, then break here
        if stop_signal.load(Ordering::SeqCst) {
            println!("Stopped fetching data since interrupted");
            break;
        }

        match fetch_data(&path).await {
            Ok(data) => {
                // println!("emitting: {}", data);
                let _ = app.emit_all("list_data", json!({"children": data, "parent": path}));

                for item in &data {
                    if let Some(fp) = &item.folder_path {
                        paths.push(fp.clone());
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
