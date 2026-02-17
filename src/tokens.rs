// Token cost calculation
// Wave 1: agent-schedule implementation

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u64>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u64>,
}

pub fn calculate_cost(model: &str, usage: &TokenUsage) -> f64 {
    let (input_per_m, output_per_m) = get_pricing(model);

    // Standard tokens
    let base_cost = (usage.input_tokens as f64 / 1_000_000.0) * input_per_m
        + (usage.output_tokens as f64 / 1_000_000.0) * output_per_m;

    // Cache creation (1.25x input price)
    let cache_creation_cost = usage.cache_creation_input_tokens
        .map(|t| (t as f64 / 1_000_000.0) * input_per_m * 1.25)
        .unwrap_or(0.0);

    // Cache reads (0.1x input price)
    let cache_read_cost = usage.cache_read_input_tokens
        .map(|t| (t as f64 / 1_000_000.0) * input_per_m * 0.1)
        .unwrap_or(0.0);

    base_cost + cache_creation_cost + cache_read_cost
}

fn get_pricing(model: &str) -> (f64, f64) {
    // Returns (input_per_million, output_per_million) in USD
    let lower = model.to_lowercase();

    if lower.contains("opus") {
        (15.0, 75.0)
    } else if lower.contains("sonnet") {
        (3.0, 15.0)
    } else if lower.contains("haiku") {
        (1.0, 5.0)
    } else {
        (3.0, 15.0)  // Default to Sonnet pricing
    }
}
