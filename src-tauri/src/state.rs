use shared::document::Document;
use tokio::sync::Mutex;

use crate::fetcher_manager::FetcherManager;

pub struct AppState {
    pub fetcher_manager: FetcherManager,
    pub document: Document,
}

pub type SharedState = Mutex<AppState>;
