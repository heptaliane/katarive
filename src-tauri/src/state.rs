use shared::document::Document;
use tokio::sync::Mutex;

use crate::fetcher_manager::FetcherManager;
use crate::speaker_manager::SpeakerManager;

pub struct AppState {
    pub fetcher_manager: FetcherManager,
    pub speaker_manager: SpeakerManager,
    pub document: Document,
}

pub type SharedState = Mutex<AppState>;
