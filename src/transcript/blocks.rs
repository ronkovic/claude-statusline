// 5-hour session block detection
// Wave 1: agent-transcript implementation

use chrono::{DateTime, Local, Duration};
use super::message::TranscriptMessage;

const SESSION_GAP_HOURS: i64 = 5;

#[derive(Debug, Clone)]
pub struct BlockInfo {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub message_count: usize,
}

pub fn detect_blocks(messages: &[TranscriptMessage]) -> Vec<BlockInfo> {
    let mut blocks = Vec::new();
    let mut current_block: Option<BlockInfo> = None;

    for msg in messages {
        if let Some(dt) = msg.timestamp_dt() {
            match &mut current_block {
                None => {
                    current_block = Some(BlockInfo { start: dt, end: dt, message_count: 1 });
                }
                Some(block) => {
                    if dt.signed_duration_since(block.end) > Duration::hours(SESSION_GAP_HOURS) {
                        blocks.push(block.clone());
                        current_block = Some(BlockInfo { start: dt, end: dt, message_count: 1 });
                    } else {
                        block.end = dt;
                        block.message_count += 1;
                    }
                }
            }
        }
    }

    if let Some(block) = current_block {
        blocks.push(block);
    }

    blocks
}
