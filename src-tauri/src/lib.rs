mod commands;
mod fetcher_manager;
mod pb;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::fetcher::fetch_document,
            commands::speaker::start_speech,
            commands::speaker::pause_speech,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
