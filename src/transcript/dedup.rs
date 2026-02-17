// HashSet-based message deduplication
// Wave 1: agent-transcript implementation

use super::message::TranscriptMessage;
use std::collections::HashSet;

pub fn deduplicate(messages: Vec<TranscriptMessage>) -> Vec<TranscriptMessage> {
    let mut seen = HashSet::new();
    messages.into_iter().filter(|msg| {
        if let Some(ts) = &msg.timestamp {
            seen.insert(ts.clone())
        } else {
            true
        }
    }).collect()
}
