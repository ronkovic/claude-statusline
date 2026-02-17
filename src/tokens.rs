// Token cost calculation
// Wave 1: agent-schedule implementation

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

pub fn calculate_cost(_model: &str, _usage: &TokenUsage) -> f64 {
    // Stub: agent-schedule implements
    0.0
}
