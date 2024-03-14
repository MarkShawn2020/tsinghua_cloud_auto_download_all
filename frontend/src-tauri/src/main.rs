// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{AppHandle, command, Manager, State};

mod schema;
mod utils;

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

    let (s, r) = async_channel::bounded(4);

    let app_cloned = app.clone();
    let store_path_cloned = store_path.clone();
    let repo_cloned = repo.clone();

    tokio::spawn(async move {
        while let Ok(path) = r.recv().await {
            utils::consumer(app_cloned.clone(), store_path_cloned.clone(), repo_cloned.clone(), path).await.unwrap();
        }
    });

    tokio::spawn(async move {
        utils::producer(app.clone(), store_path.clone(), repo.clone(), root_path.clone(), stop_signal, s).await.expect("waiting...");
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
