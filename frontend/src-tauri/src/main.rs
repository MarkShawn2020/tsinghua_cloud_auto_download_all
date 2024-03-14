// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::c_int;
use std::future::Future;
use std::pin::Pin;
use std::ptr::null;
use reqwest;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use serde_json::{from_str, json};
use tauri::{AppHandle, command, Manager};
use threadpool::ThreadPool;
use tokio::runtime::Runtime;

use serde::{Serialize, Deserialize};
use serde::de::Error;


#[derive(Serialize, Deserialize)]
struct DirItem {
    size: u128,
    is_dir: bool,
    last_modified: String,
    file_path: Option<String>,
    file_name: Option<String>,
    folder_name: Option<String>,
    folder_path: Option<String>,
}

type DirList = Vec<DirItem>;

#[derive(Serialize, Deserialize)]
struct ListData {
    dirent_list: DirList,
}

type TheError = Box<dyn std::error::Error + Send + Sync>;

async fn fetch_data(path: &str) -> Result<DirList, TheError> {
    println!("fetching data...");
    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/689824200edb49888695/dirents?path={}", path))
        .await?
        .text()
        .await?;

    let p: DirList = from_str::<ListData>(&response)?.dirent_list;

    Ok(p)
}

async fn recursive_fetch_and_emit(app: &AppHandle, root_path: &str) {
    let mut paths = vec![root_path.to_owned()];

    while let Some(path) = paths.pop() {
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


#[command]
async fn fetch_data_and_emit(app: AppHandle, path: String) {
    println!("fetching data and emit...");
    recursive_fetch_and_emit(&app, &path).await;
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch_data_and_emit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
