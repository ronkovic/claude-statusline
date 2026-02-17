// TranscriptMessage conversion with usage compatibility
// Wave 1: agent-transcript implementation

use serde::Deserialize;
use crate::tokens::TokenUsage;

#[derive(Debug, Deserialize)]
pub struct TranscriptMessage {
    pub timestamp: Option<String>,
    pub usage: Option<TokenUsage>,
}

pub fn parse_message(_line: &str) -> Option<TranscriptMessage> {
    // Stub
    None
}
