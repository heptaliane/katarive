use shared::document::Document;
use tokio::sync::Mutex;

mod commands;
mod fetcher_manager;
mod pb;
mod speaker_manager;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let fm = fetcher_manager::FetcherManager::new();
    // TODO: Register fetchers
    let sm = speaker_manager::SpeakerManager::new();
    // TODO: Register speakers

    let state = Mutex::new(state::AppState {
        fetcher_manager: fm,
        speaker_manager: sm,
        document: Document::default(),
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::fetcher::fetch_document,
            commands::speaker::start_speech,
            commands::speaker::pause_speech,
            commands::speaker::resume_speech,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
