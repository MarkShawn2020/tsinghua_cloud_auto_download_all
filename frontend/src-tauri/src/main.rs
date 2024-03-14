// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use serde_json::json;
use tauri::{AppHandle, command, Manager, State};

mod schema;
mod utils;

async fn recursive_fetch_and_emit(app: &AppHandle, root_path: &str, stop_signal: State<'_, Arc<AtomicBool>>) {
    let mut paths = vec![root_path.to_owned()];

    while let Some(path) = paths.pop() {

        // how to: if received STOP signal from another invoke, then break here
        if stop_signal.load(Ordering::SeqCst) {
            println!("Stopped fetching data since interrupted");
            break;
        }

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
async fn fetch_data_and_emit(app: AppHandle, path: String, stop_signal: State<'_, Arc<AtomicBool>>) -> Result<(), String>{
    println!("fetching data and emit...");
    stop_signal.store(false, Ordering::SeqCst);
    recursive_fetch_and_emit(&app, &path, stop_signal).await;
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

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch_data_and_emit, stop_fetching])
        .manage(stop_signal)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
