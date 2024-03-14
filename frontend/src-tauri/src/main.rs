// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{AppHandle, command, Manager, State};

mod schema;
mod utils;


#[command]
async fn fetch_data_and_emit(app: AppHandle, store_path: Box<Path>, path: String, stop_signal: State<'_, Arc<AtomicBool>>) -> Result<(), String> {
    println!("fetching data and emit...");
    stop_signal.store(false, Ordering::SeqCst);
    utils::recursive_fetch_and_emit(&app, &store_path, &path, stop_signal).await;
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
