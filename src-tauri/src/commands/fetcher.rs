use log;
use shared::document::{Document, FetchDocumentArgs};

use crate::state::SharedState;

#[tauri::command]
pub async fn fetch_document(
    args: FetchDocumentArgs,
    state: tauri::State<'_, SharedState>,
) -> Result<Document, String> {
    let mut state = state.lock().await;
    if let Some(fetcher) = state.fetcher_manager.find_client(&args.url) {
        match fetcher.fetch(&args.url).await {
            Ok(res) => {
                let document = Document {
                    title: res.title,
                    body: res.content,
                };
                state.document = document.clone();
                Ok(document)
            }
            Err(err) => {
                log::warn!("Failed to fetch document: {:?}", err.to_string());
                Err(err.to_string())
            }
        }
    } else {
        Err(format!("Unsupported url: '{:?}'", args.url))
    }
}
