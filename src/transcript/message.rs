// TranscriptMessage conversion with usage compatibility
// Wave 1: agent-transcript implementation

use serde::Deserialize;
use crate::tokens::TokenUsage;

#[derive(Debug, Deserialize, Clone)]
pub struct TranscriptMessage {
    pub timestamp: Option<String>,
    pub usage: Option<TokenUsage>,
}

pub fn parse_message(line: &str) -> Option<TranscriptMessage> {
    serde_json::from_str(line).ok()
}

impl TranscriptMessage {
    pub fn timestamp_dt(&self) -> Option<chrono::DateTime<chrono::Local>> {
        self.timestamp.as_ref().and_then(|ts| {
            chrono::DateTime::parse_from_rfc3339(ts)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Local))
        })
    }
}
