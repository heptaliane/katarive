use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum SidecarMessage {
    Ready { port: u16 },
    Status { level: String, message: String },
}
