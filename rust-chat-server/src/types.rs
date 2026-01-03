use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    content: String,
    sender: String,
    timestamp: u64,
}

impl ChatMessage {
    pub fn format(&self) -> String {
        format!("[{}] {}: {}", self.timestamp, self.sender, self.content)
    }
}
