use tokio::sync::Mutex;

use crate::fetcher_manager::FetcherManager;

pub struct AppState {
    pub fetcher_manager: FetcherManager,
    pub document: Document,
}

pub type SharedState = Mutex<AppState>;

pub struct Document {
    pub title: String,
    pub content: Vec<String>,
}

impl Default for Document {
    fn default() -> Self {
        Document {
            title: "".to_string(),
            content: Vec::new(),
        }
    }
}
