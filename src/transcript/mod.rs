// Transcript processing module
// Wave 1: agent-transcript implements all

pub mod finder;
pub mod reader;
pub mod message;
pub mod dedup;
pub mod blocks;
pub mod stats;
pub mod burn;

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub total_input: u64,
    pub total_output: u64,
    pub message_count: usize,
    pub block_count: usize,
    pub block_start: Option<chrono::DateTime<chrono::Local>>,
    pub block_end: Option<chrono::DateTime<chrono::Local>>,
    pub burn_timeline: Vec<u64>,
    pub total_cache_creation: u64,
    pub total_cache_read: u64,
    pub duration_seconds: i64,
}

pub fn load_and_analyze(transcript_path: Option<&str>) -> crate::error::Result<Option<SessionStats>> {
    use std::path::PathBuf;

    let files = if let Some(path) = transcript_path {
        // Use explicit transcript path from stdin
        vec![PathBuf::from(path)]
    } else {
        // Fallback to finder
        finder::find_recent_transcripts()?
    };

    let mut all_messages = Vec::new();
    for file in files {
        let lines = reader::read_transcript_lines(&file)?;
        for line in lines {
            if let Some(msg) = message::parse_message(&line) {
                all_messages.push(msg);
            }
        }
    }

    let messages = dedup::deduplicate(all_messages);
    Ok(stats::calculate_stats(&messages))
}
