// Number formatting utilities
// Wave 1: agent-display implementation

pub fn format_tokens(n: u64) -> String {
    // Stub: 999→"999", 1500→"1.5K", 14_000_000→"14.0M"
    n.to_string()
}

pub fn format_cost(cost: f64) -> String {
    // Stub
    format!("${:.2}", cost)
}
