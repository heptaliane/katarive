use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Clone, Deserialize, Serialize)]
pub struct SpeakerStartArgs {
    pub name: String,
    pub options: HashMap<String, String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SpeakerControlArgs {
    pub name: String,
}
