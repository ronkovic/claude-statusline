use crate::colors::*;
use crate::display::format::format_tokens_short;
use crate::display::progress::render_sparkline;
use crate::display::DisplayContext;

pub fn render(ctx: &DisplayContext, _max_width: usize) -> String {
    let stats = match &ctx.stats {
        Some(s) => s,
        None => return String::new(),
    };

    let timeline = &stats.burn_timeline;
    if timeline.is_empty() {
        return String::new();
    }

    // Total tokens (w/cache)
    let total_tokens: u64 = timeline.iter().sum();

    // Calculate rate (tokens/minute)
    let duration_seconds = if let Some(block_start) = stats.block_start {
        use chrono::Local;
        let now = Local::now();
        (now - block_start).num_seconds()
    } else {
        0
    };

    let rate = if duration_seconds > 0 {
        (total_tokens as f64 / duration_seconds as f64 * 60.0) as u64
    } else {
        0
    };

    // Label (aligned with "Session:" and "Compact:")
    let label = format!("{BRIGHT_CYAN}Burn:   {RESET}");

    // Sparkline
    let sparkline = render_sparkline(timeline, 20);

    // Token count
    let tokens_str = format!("{BRIGHT_WHITE}{} token(w/cache){RESET}",
        format_tokens_short(total_tokens)
    );

    // Rate
    let rate_str = format!("{BRIGHT_WHITE}Rate: {} t/m{RESET}",
        format_tokens_short(rate)
    );

    format!("{} {} {}, {}", label, sparkline, tokens_str, rate_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StdinData;
    use crate::transcript::SessionStats;
    use chrono::Local;

    #[test]
    fn test_render_line4_with_burn_data() {
        let now = Local::now();
        let block_start = now - chrono::Duration::minutes(20);

        let burn_timeline = vec![100000, 200000, 300000, 150000, 250000];

        let stats = SessionStats {
            total_input: 10000,
            total_output: 5000,
            message_count: 10,
            block_count: 1,
            block_start: Some(block_start),
            block_end: Some(now),
            burn_timeline,
            total_cache_creation: 0,
            total_cache_read: 0,
            duration_seconds: 1200,
        };

        let ctx = DisplayContext::new(
            StdinData::default(),
            Some(stats),
            80,
            None,
            None,
        );

        let result = render(&ctx, 80);
        assert!(result.contains("Burn:"));
        assert!(result.contains("token(w/cache)"));
        assert!(result.contains("Rate:"));
        assert!(result.contains("t/m"));
    }

    #[test]
    fn test_render_line4_without_stats() {
        let ctx = DisplayContext::new(
            StdinData::default(),
            None,
            80,
            None,
            None,
        );

        let result = render(&ctx, 80);
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_line4_empty_timeline() {
        let stats = SessionStats {
            total_input: 10000,
            total_output: 5000,
            message_count: 10,
            block_count: 1,
            block_start: None,
            block_end: None,
            burn_timeline: vec![],
            total_cache_creation: 0,
            total_cache_read: 0,
            duration_seconds: 0,
        };

        let ctx = DisplayContext::new(
            StdinData::default(),
            Some(stats),
            80,
            None,
            None,
        );

        let result = render(&ctx, 80);
        assert_eq!(result, "");
    }

    #[test]
    fn test_render_line4_rate_calculation() {
        let now = Local::now();
        let block_start = now - chrono::Duration::seconds(60); // 1 minute

        // 60,000 tokens in 1 minute = 60,000 tokens/minute
        let burn_timeline = vec![60000];

        let stats = SessionStats {
            total_input: 10000,
            total_output: 5000,
            message_count: 10,
            block_count: 1,
            block_start: Some(block_start),
            block_end: Some(now),
            burn_timeline,
            total_cache_creation: 0,
            total_cache_read: 0,
            duration_seconds: 60,
        };

        let ctx = DisplayContext::new(
            StdinData::default(),
            Some(stats),
            80,
            None,
            None,
        );

        let result = render(&ctx, 80);
        // Rate should be approximately 60,000 t/m
        assert!(result.contains("60") || result.contains("59") || result.contains("61"));
    }
}
