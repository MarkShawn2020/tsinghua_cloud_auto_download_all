// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::c_int;
use reqwest;
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
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

#[derive(Serialize, Deserialize)]
struct ListData {
    dirent_list: Vec<DirItem>,
}

type TheError = Box<dyn std::error::Error + Send + Sync>;

async fn fetch_data(path: &str) -> Result<ListData, TheError> {
    println!("fetching data...");
    let response = reqwest::get(format!("https://cloud.tsinghua.edu.cn/api/v2.1/share-links/689824200edb49888695/dirents?path={}", path))
        .await?
        .text()
        .await?;

    let p: ListData = from_str::<ListData>(&response)?;

    Ok(p)
}


#[command]
fn fetch_data_and_emit(app: AppHandle, path: &str) {
    println!("fetching data and emit...");

    let app_clone = app.clone();

    let rt = Runtime::new().unwrap(); // new tokio runtime

    rt.block_on(async move {
        match fetch_data(path).await {
            Ok(data) => {
                // println!("emitting: {}", data);
                let _ = app_clone.emit_all("list_data", json!({"data": data}));
            }
            Err(e) => eprintln!("Error fetching data: {}", e)
        }
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_data_and_emit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
