use std::fs::create_dir_all;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::stream::StreamExt;
use serde_json::{from_str, json};
use tauri::{AppHandle, Manager};
use tokio::io::AsyncWriteExt;

use crate::schema::ListData;

pub async fn producer(
    app: AppHandle,
    store_path: String,
    repo: String,
    root_path: String,
    stop_signal: Arc<AtomicBool>,
    s: async_channel::Sender<String>)
    -> Result<(), Box<dyn std::error::Error>> {
    let mut paths = vec![root_path.clone()];

    while let Some(path) = paths.pop() {
        if stop_signal.load(Ordering::SeqCst) {
            println!("stopped since interrupted");
            break;
        }

        create_dir_all(Path::new(&store_path.clone()).join(path.clone().strip_prefix("/").unwrap())).unwrap();

        println!("-- listing {}", path);

        let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/{}/dirents?path={}", repo, path)).await?.text().await?;

        let data = from_str::<ListData>(&response)?.dirent_list;

        if let Some(items) = data {
            app.emit_all("core", json!({"children": &items, "parent": &path, "type": "list-dirs"})).unwrap();

            for item in &items {
                if let Some(fp) = &item.folder_path {
                    paths.push(fp.clone());
                } else if let Some(fp) = &item.file_path {
                    s.send(fp.clone()).await.expect("TODO: panic message");
                }
            }
        }
    }

    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(())
}

pub async fn consumer(app: AppHandle, store_path: String, repo: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    println!(">> file downloading {:?}", path);

    let fps = 30;

    let mut downloaded: u64 = 0;

    app.emit_all("core", json!({"type": "file-mutation", "filePath": path, "status": "preparing", "downloaded": downloaded})).unwrap();

    let client = reqwest::Client::new();

    let resp = client.get(format!("https://cloud.tsinghua.edu.cn/d/{}/files/?p={}&dl=1", repo, path)).send().await.unwrap();

    if !resp.status().is_success() {
        app.emit_all("core", json!({"type": "file-mutation", "filePath": path, "status": "failed", "downloaded": downloaded})).unwrap();
    } else {
        // let total_size = resp.content_length().unwrap_or(0);

        let local_path = Path::new(&store_path).join(path.clone().strip_prefix("/").unwrap());

        let mut file = tokio::fs::File::create(local_path).await?;

        let mut stream = resp.bytes_stream();

        let mut t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        while let Some(item) = stream.next().await {
            let chunk = item.expect("error while downloading file");

            file.write_all(&chunk).await?;

            downloaded += chunk.len() as u64;

            let t2 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

            if t2 - t > Duration::from_millis(1000 / fps) {
                t = t2;
                app.emit_all("core", json!({"type": "file-mutation", "filePath": path, "status": "downloading", "downloaded": downloaded })).unwrap();
            }

            // DONT log, o.w. blocked...
            // println!("  downloading {} {}/{}, {:.2}%", path, downloaded, total_size, (downloaded as f64) / (total_size as f64) * 100.);
        }

        app.emit_all("core", json!({"type": "file-mutation", "filePath": path, "status": "downloaded", "downloaded": downloaded})).unwrap();
    }

    println!("<< file downloaded into {:?}", path);

    Ok(())
}
