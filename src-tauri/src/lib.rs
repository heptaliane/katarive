use tokio::sync::Mutex;

mod commands;
mod fetcher_manager;
mod pb;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let fetcher_manager = fetcher_manager::FetcherManager::new();
    // TODO: Register fetchers

    let state = Mutex::new(state::AppState {
        document: state::Document::default(),
        fetcher_manager,
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::fetcher::fetch_document,
            commands::speaker::start_speech,
            commands::speaker::pause_speech,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
