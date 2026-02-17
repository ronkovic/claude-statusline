// Progress bar and sparkline rendering
// Wave 1: agent-display implementation

pub fn render_progress_bar(current: u64, max: u64, width: usize) -> String {
    // Stub: color thresholds (green/yellow/red)
    format!("[{:width$}]", "", width = width)
}

pub fn render_sparkline(data: &[u64]) -> String {
    // Stub: ▁▂▃▄▅▆▇█
    "▁▁▁▁".to_string()
}
