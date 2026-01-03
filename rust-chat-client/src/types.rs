use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage<'a> {
    content: String,
    sender: &'a str,
    timestamp: u64,
}

impl<'a> ChatMessage<'a> {
    pub fn new(content: String, sender: &'a str) -> Self {
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
        format!("[{}] {}: {}", self.timestamp, self.sender, self.content)
    }
}
