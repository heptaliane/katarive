use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Document {
    pub title: String,
    pub body: Vec<String>,
}
