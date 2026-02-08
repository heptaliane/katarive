use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Document {
    pub title: String,
    pub body: Vec<String>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            body: Vec::new(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct FetchDocumentArgs {
    pub url: String,
}
