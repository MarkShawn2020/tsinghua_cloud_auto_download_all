// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::c_int;
use reqwest;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use serde_json::json;
use tauri::{AppHandle, command, Manager};
use threadpool::ThreadPool;
use tokio::runtime::Runtime;

struct DirItem {
    size: c_int,
    is_dir: bool,
    last_modified: String,
    folder_path: String,
    folder_name: String,
}

struct ListData {
    dirent_list: Vec<DirItem>,
}

async fn fetch_data() -> Result<String, reqwest::Error> {
    println!("fetching data...");
    let response = reqwest::get("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/689824200edb49888695/dirents?path=/")
        .await?
        .text()
        .await?;
    Ok(response)
}

#[command]
fn fetch_data_and_emit(app: AppHandle) {
    println!("fetching data and emit...");

    let app_clone = app.clone();
    // new tokio runtime
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        match fetch_data().await {
            Ok(data) => {
                println!("emitting: {}", data);
                let _ = app_clone.emit_all("list_data", json!({"data": data}));
            }
            Err(e) => eprintln!("Error fetching data {:?}", e)
        }
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_data_and_emit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
