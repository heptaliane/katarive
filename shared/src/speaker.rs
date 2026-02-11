use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct SpeakerMetadata {
    pub name: String,
    pub version: String,
    pub options: Vec<SpeakerOption>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SpeakerOption {
    pub id: String,
    pub label: String,
    pub description: String,
    pub default_value: String,
}
