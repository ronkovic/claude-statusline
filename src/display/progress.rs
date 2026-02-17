// Progress bar and sparkline rendering
use crate::colors::*;

pub fn render_progress_bar(value: u64, max: u64, width: usize) -> String {
    let percentage = if max > 0 {
        (value as f64 / max as f64 * 100.0) as u64
    } else {
        0
    };

    let filled = (value as f64 / max as f64 * width as f64) as usize;
    let empty = width.saturating_sub(filled);

    let color = get_percentage_color(percentage);
    let bar = format!("{}{}{}", "█".repeat(filled), "▒".repeat(empty), RESET);

    format!("{}{}{}", color, bar, RESET)
}

fn get_percentage_color(percentage: u64) -> &'static str {
    if percentage >= 85 {
        BG_RED
    } else if percentage >= 70 {
        BRIGHT_YELLOW
    } else {
        BRIGHT_GREEN
    }
}

pub fn render_session_bar(percentage: f64, width: usize) -> String {
    let filled = (percentage / 100.0 * width as f64) as usize;
    let current_pos = filled.min(width.saturating_sub(1));

    let mut bar = String::new();
    for i in 0..width {
        if i < current_pos {
            bar.push_str(&format!("{}{}{}", BRIGHT_GREEN, "█", RESET));
        } else if i == current_pos {
            bar.push_str(&format!("{}{}{}", BRIGHT_WHITE, "▓", RESET));
        } else {
            bar.push_str(&format!("{}{}{}", LIGHT_GRAY, "▒", RESET));
        }
    }
    bar
}

pub fn render_sparkline(values: &[u64], width: usize) -> String {
    if values.is_empty() || values.iter().all(|&v| v == 0) {
        return format!("{}{}{}", LIGHT_GRAY, "▁".repeat(width), RESET);
    }

    let max_val = *values.iter().max().unwrap();
    if values.iter().all(|&v| v == max_val) {
        return format!("{}{}{}", BRIGHT_GREEN, "▅".repeat(width), RESET);
    }

    let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let step = width.max(1);
    let chunk_size = values.len() / step;

    let mut result = String::new();
    for i in 0..width {
        let start = i * chunk_size;
        let end = ((i + 1) * chunk_size).min(values.len());
        let chunk_max = values[start..end].iter().max().copied().unwrap_or(0);

        let normalized = chunk_max as f64 / max_val as f64;
        let block_idx = ((normalized * (blocks.len() - 1) as f64).round() as usize).min(blocks.len() - 1);

        let color = if normalized > 0.7 {
            BRIGHT_RED
        } else if normalized > 0.4 {
            BRIGHT_YELLOW
        } else {
            BRIGHT_GREEN
        };

        result.push_str(&format!("{}{}{}", color, blocks[block_idx], RESET));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_progress_bar() {
        let bar = render_progress_bar(50, 100, 10);
        assert!(!bar.is_empty());
        // Bar should contain filled and empty portions
        assert!(bar.contains("█"));
        assert!(bar.contains("▒"));
    }

    #[test]
    fn test_render_progress_bar_full() {
        let bar = render_progress_bar(100, 100, 10);
        // Full bar should contain only filled blocks
        assert!(bar.contains("█"));
    }

    #[test]
    fn test_render_progress_bar_empty() {
        let bar = render_progress_bar(0, 100, 10);
        // Empty bar should contain only empty blocks
        assert!(bar.contains("▒"));
    }

    #[test]
    fn test_render_session_bar() {
        let bar = render_session_bar(50.0, 10);
        assert!(!bar.is_empty());
        assert!(bar.contains("█") || bar.contains("▓"));
    }

    #[test]
    fn test_render_sparkline() {
        let values = vec![1, 2, 3, 4, 5];
        let sparkline = render_sparkline(&values, 5);
        assert!(!sparkline.is_empty());
    }

    #[test]
    fn test_render_sparkline_empty() {
        let values: Vec<u64> = vec![];
        let sparkline = render_sparkline(&values, 5);
        assert!(!sparkline.is_empty());
    }

    #[test]
    fn test_render_sparkline_all_zeros() {
        let values = vec![0, 0, 0, 0];
        let sparkline = render_sparkline(&values, 4);
        assert!(!sparkline.is_empty());
    }
}
