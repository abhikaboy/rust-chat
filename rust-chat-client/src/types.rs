use serde::{Serialize,Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    content: String,
    sender: String,
    timestamp: u64,
}

impl ChatMessage {
    pub fn new(content: String, sender: String) -> Self {
        Self {
            content,
            sender,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }



    pub fn format(&self) -> String {
        format!("[{}] {}: {}", format_timestamp(self.timestamp), self.sender, self.content)
    }
}
fn format_timestamp(timestamp: u64) -> String {
    let dt = DateTime::<Utc>::from_timestamp(timestamp as i64, 0)
        .expect("Invalid timestamp");

    dt.format("%H:%M:%S").to_string() // HH:MM:SS
}
