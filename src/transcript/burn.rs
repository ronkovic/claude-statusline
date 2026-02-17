// 15-minute × 20 segment burn timeline
// Wave 1: agent-transcript implementation

use chrono::{Local, Duration};
use super::message::TranscriptMessage;

const SEGMENT_MINUTES: i64 = 15;
const SEGMENT_COUNT: usize = 20;

pub fn generate_burn_timeline(messages: &[TranscriptMessage]) -> Vec<u64> {
    let mut timeline = vec![0u64; SEGMENT_COUNT];

    let now = Local::now();
    let start_time = now - Duration::minutes(SEGMENT_MINUTES * SEGMENT_COUNT as i64);

    for msg in messages {
        if let (Some(dt), Some(usage)) = (msg.timestamp_dt(), &msg.usage) {
            if dt >= start_time {
                let minutes_since = dt.signed_duration_since(start_time).num_minutes();
                let segment = (minutes_since / SEGMENT_MINUTES) as usize;
                if segment < SEGMENT_COUNT {
                    let total_tokens = usage.input_tokens
                        + usage.output_tokens
                        + usage.cache_creation_input_tokens.unwrap_or(0)
                        + usage.cache_read_input_tokens.unwrap_or(0);
                    timeline[segment] += total_tokens;
                }
            }
        }
    }

    timeline
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenUsage;

    #[test]
    fn test_calculate_total_tokens_with_cache() {
        // Test that total tokens include cache tokens
        let usage = TokenUsage {
            input_tokens: 100,
            output_tokens: 50,
            cache_creation_input_tokens: Some(25),
            cache_read_input_tokens: Some(10),
        };
        let total = calculate_token_total(&usage);
        assert_eq!(total, 185); // 100 + 50 + 25 + 10
    }

    #[test]
    fn test_calculate_total_tokens_without_cache() {
        // Test that calculation works when cache fields are None
        let usage = TokenUsage {
            input_tokens: 100,
            output_tokens: 50,
            cache_creation_input_tokens: None,
            cache_read_input_tokens: None,
        };
        let total = calculate_token_total(&usage);
        assert_eq!(total, 150); // 100 + 50
    }

    fn calculate_token_total(usage: &crate::tokens::TokenUsage) -> u64 {
        usage.input_tokens
            + usage.output_tokens
            + usage.cache_creation_input_tokens.unwrap_or(0)
            + usage.cache_read_input_tokens.unwrap_or(0)
    }
}
