// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::c_int;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

use futures_util::future::join_all;
use serde_json::{from_str, json};
use tauri::{AppHandle, command, Manager, State};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use utils::download_file;

use crate::schema::{DirList, ListData, TheError};

mod schema;
mod utils;


async fn producer(app: AppHandle, repo: String, root_path: String, stop_signal: Arc<AtomicBool>, tx: broadcast::Sender<(String, c_int)>) {
    let mut paths = vec![root_path.clone()];
    let mut index = 0;

    while let Some(path) = paths.pop() {
        if (stop_signal.load(Ordering::SeqCst)) {
            println!("stopped since interrupted");
            break;
        }

        match utils::fetch_dir_list(repo.clone(), path.clone()).await {
            Ok(data) => {
                // println!("emitting: {}", data);
                let _ = app.emit_all("list_data", json!({"children": &data, "parent": &path}));

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

async fn consumer(store_path: String, repo: String, path: String) {
    download_file(store_path.clone(), repo.clone(), path.clone()).await.expect("TODO: panic message");
}

/***
todo: static

ref:
- https://chat.openai.com/c/cfa0aba4-16c4-4aea-9e6f-4bba0e56a507
- https://chat.openai.com/c/d428f113-52d1-408d-9986-52abc20ab04a

 */
#[command]
async fn fetch_data_and_emit(
    app: AppHandle,
    store_path: String,
    repo: String,
    root_path: String,
    stop_signal: State<'_, Arc<AtomicBool>>,
) -> Result<(), String> {
    println!("fetching data and emit...");

    let stop_signal = Arc::clone(&stop_signal);
    stop_signal.store(false, Ordering::SeqCst);

    let (tx, _): (broadcast::Sender<(String, c_int)>, broadcast::Receiver<(String, c_int)>) = broadcast::channel(120);

    let n = 5;
    for i in 0..n {
        let mut rx = tx.subscribe();
        let repo = repo.clone();
        let store_path = store_path.clone();

        tokio::spawn(async move {
            while let Ok((path, index)) = rx.recv().await {
                if (index % n == i) {
                    consumer(store_path.clone(), repo.clone(), path).await;
                }
            }
        });
    }


    tokio::spawn(async move {
        producer(app.clone(), repo.clone(), root_path.clone(), stop_signal, tx).await;
    }).await.unwrap();


    Ok(())
}

#[command]
async fn stop_fetching(stop_signal: State<'_, Arc<AtomicBool>>) -> Result<(), String> {
    stop_signal.store(true, Ordering::SeqCst);
    println!("Stop signal set!");
    Ok(())
}

fn main() {
    let stop_signal = Arc::new(AtomicBool::new(false));

    tauri::Builder::default().setup(|app| {
        #[cfg(debug_assertions)]
        {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
        }
        Ok(())
    }).invoke_handler(tauri::generate_handler![fetch_data_and_emit, stop_fetching]).manage(stop_signal).run(tauri::generate_context!()).expect("error while running tauri application");
}
