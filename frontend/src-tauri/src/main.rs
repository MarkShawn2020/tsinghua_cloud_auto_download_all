// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use serde_json::json;
use tauri::{AppHandle, command, Manager};

mod schema;
mod utils;

async fn recursive_fetch_and_emit(app: &AppHandle, root_path: &str) {
    let mut paths = vec![root_path.to_owned()];

    while let Some(path) = paths.pop() {

        // how to: if received STOP signal from another invoke, then break here

        match utils::fetch_data(&path).await {
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
