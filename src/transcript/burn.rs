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
                    timeline[segment] += usage.output_tokens;
                }
            }
        }
    }

    timeline
}
