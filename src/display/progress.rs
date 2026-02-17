// Progress bar and sparkline rendering
use crate::colors::*;

pub fn render_progress_bar(current: u64, max: u64, width: usize) -> String {
    let pct = if max > 0 {
        current as f64 / max as f64
    } else {
        0.0
    };
    let filled = ((width as f64) * pct) as usize;

    let color = if pct >= 0.85 {
        BG_RED
    } else if pct >= 0.70 {
        YELLOW
    } else {
        GREEN
    };

    let bar = "█".repeat(filled) + &"░".repeat(width.saturating_sub(filled));
    c(color, &bar)
}

pub fn render_sparkline(data: &[u64]) -> String {
    const CHARS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    if data.is_empty() {
        return String::new();
    }

    let max = *data.iter().max().unwrap_or(&1);
    if max == 0 {
        return CHARS[0].to_string().repeat(data.len());
    }

    data.iter()
        .map(|&v| {
            let idx = ((v as f64 / max as f64) * 7.0) as usize;
            CHARS[idx.min(7)]
        })
        .collect()
}
