use shared::document::Document;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

mod commands;
mod fetcher_manager;
mod pb;
mod sidecar;
mod speaker_manager;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let fm = fetcher_manager::FetcherManager::new();
    let sm = speaker_manager::SpeakerManager::new();
    // TODO: Register speakers

    let state = Arc::new(Mutex::new(state::AppState {
        fetcher_manager: fm,
        speaker_manager: sm,
        document: Document::default(),
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .manage(state.clone())
        .invoke_handler(tauri::generate_handler![
            commands::fetcher::fetch_document,
            commands::speaker::start_speech,
            commands::speaker::pause_speech,
            commands::speaker::resume_speech,
        ])
        .setup(move |app| {
            let handle = app.handle().clone();
            let state = state.clone();
            tauri::async_runtime::spawn(async move {
                let mut state = state.lock().await;
                let fetchers = vec!["echo_fetcher"];
                for name in fetchers {
                    match sidecar::spawn_sidecar(&handle, name).await {
                        Ok(addr) => {
                            match state.fetcher_manager.add_client(&format!("{}", addr)).await {
                                Err(err) => log::error!("Failed to setup fetcher: {}", err),
                                _ => (),
                            }
                        }
                        Err(err) => log::error!("Failed to start sidecar: {}", err),
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
