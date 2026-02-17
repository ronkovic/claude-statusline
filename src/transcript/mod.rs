// Transcript processing module
// Wave 1: agent-transcript implements all

pub mod finder;
pub mod reader;
pub mod message;
pub mod dedup;
pub mod blocks;
pub mod stats;
pub mod burn;

use crate::tokens::TokenUsage;

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub total_input: u64,
    pub total_output: u64,
    pub message_count: usize,
    pub block_count: usize,
    pub block_start: Option<chrono::DateTime<chrono::Local>>,
    pub block_end: Option<chrono::DateTime<chrono::Local>>,
    pub burn_timeline: Vec<u64>,
}

pub fn load_and_analyze() -> crate::error::Result<Option<SessionStats>> {
    // Stub: agent-transcript implements full pipeline
    Ok(None)
}
