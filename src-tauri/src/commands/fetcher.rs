use log;

use crate::state::{Document, SharedState};

#[tauri::command]
pub async fn fetch_document(
    url: String,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    if let Some(fetcher) = state.fetcher_manager.find_client(&url) {
        match fetcher.fetch(&url).await {
            Ok(res) => {
                state.document = Document {
                    title: res.title,
                    content: res.content,
                };
                Ok(())
            }
            Err(err) => {
                log::warn!("Failed to fetch document: {:?}", err.to_string());
                Err(err.to_string())
            }
        }
    } else {
        Err(format!("Unsupported url: '{:?}'", url))
    }
}
