// 5-hour session block detection
// Wave 1: agent-transcript implementation

use chrono::{DateTime, Local};
use super::message::TranscriptMessage;

pub fn detect_blocks(_messages: &[TranscriptMessage]) -> Vec<BlockInfo> {
    // Stub
    vec![]
}

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub message_count: usize,
}
