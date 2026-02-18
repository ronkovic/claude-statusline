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

    let total_cache_creation: u64 = messages.iter()
        .filter_map(|m| m.usage.as_ref())
        .map(|u| u.cache_creation_input_tokens.unwrap_or(0))
        .sum();

    let total_cache_read: u64 = messages.iter()
        .filter_map(|m| m.usage.as_ref())
        .map(|u| u.cache_read_input_tokens.unwrap_or(0))
        .sum();

    let blocks = detect_blocks(messages);
    let current_block = blocks.last()?;

    let burn_timeline = generate_burn_timeline(messages);

    // Calculate duration_seconds
    use chrono::Local;
    let now = Local::now();
    let duration_seconds = (now - current_block.start).num_seconds();

    Some(SessionStats {
        total_input,
        total_output,
        message_count: messages.len(),
        block_count: blocks.len(),
        block_start: Some(current_block.start),
        block_end: Some(current_block.end),
        burn_timeline,
        total_cache_creation,
        total_cache_read,
        duration_seconds,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenUsage;

    #[test]
    fn test_sum_cache_creation_tokens() {
        // Test that cache creation tokens are summed correctly
        let usages = vec![
            (Some(25u64), Some(10u64)),
            (Some(15u64), None),
            (None, Some(5u64)),
        ];
        let total_cache_creation = usages.iter()
            .map(|(cc, _)| cc.unwrap_or(0))
            .sum::<u64>();
        let total_cache_read = usages.iter()
            .map(|(_, cr)| cr.unwrap_or(0))
            .sum::<u64>();

        assert_eq!(total_cache_creation, 40); // 25 + 15
        assert_eq!(total_cache_read, 15);     // 10 + 5
    }

    #[test]
    fn test_empty_cache_tokens() {
        // Test that cache tokens default to 0 when None
        let usages = vec![
            (None, None),
            (None, None),
        ];
        let total_cache_creation = usages.iter()
            .map(|(cc, _)| cc.unwrap_or(0))
            .sum::<u64>();
        let total_cache_read = usages.iter()
            .map(|(_, cr)| cr.unwrap_or(0))
            .sum::<u64>();

        assert_eq!(total_cache_creation, 0);
        assert_eq!(total_cache_read, 0);
    }
}
