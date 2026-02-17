// Block statistics calculation
// Wave 1: agent-transcript implementation

use super::message::TranscriptMessage;
use super::SessionStats;
use super::blocks::detect_blocks;
use super::burn::generate_burn_timeline;

pub fn calculate_stats(messages: &[TranscriptMessage]) -> Option<SessionStats> {
    if messages.is_empty() {
        return None;
    }

    let total_input: u64 = messages.iter()
        .filter_map(|m| m.usage.as_ref())
        .map(|u| u.input_tokens)
        .sum();

    let total_output: u64 = messages.iter()
        .filter_map(|m| m.usage.as_ref())
        .map(|u| u.output_tokens)
        .sum();

    let blocks = detect_blocks(messages);
    let current_block = blocks.last()?;

    let burn_timeline = generate_burn_timeline(messages);

    Some(SessionStats {
        total_input,
        total_output,
        message_count: messages.len(),
        block_count: blocks.len(),
        block_start: Some(current_block.start),
        block_end: Some(current_block.end),
        burn_timeline,
    })
}
