use crate::colors::*;
use crate::display::format::format_duration;
use crate::display::progress::render_session_bar;
use crate::display::DisplayContext;
use chrono::{Local, Timelike};

pub fn render(ctx: &DisplayContext, _max_width: usize) -> String {
    let stats = match &ctx.stats {
        Some(s) => s,
        None => return String::new(),
    };

    // Get block_start
    let block_start = match stats.block_start {
        Some(start) => start,
        None => return String::new(),
    };

    // Time calculation
    let now = Local::now();
    let duration_seconds = (now - block_start).num_seconds();
    let block_hours = 5;
    let block_seconds = block_hours * 3600;

    // Progress
    let block_progress = ((duration_seconds as f64 / block_seconds as f64) * 100.0).min(100.0);

    // Floor block_start to hour
    let block_start_floor = block_start
        .with_minute(0).unwrap()
        .with_second(0).unwrap();
    let block_end = block_start_floor + chrono::Duration::hours(block_hours);

    // Label
    let label = format!("{BRIGHT_CYAN}Session:{RESET}");

    // Session bar
    let bar = render_session_bar(block_progress, 20);

    // Percentage
    let pct_str = format!("{BRIGHT_WHITE}[{}%]{RESET}", block_progress as u64);

    // Duration
    let duration_str = format!("{BRIGHT_WHITE}{}/{}h{RESET}",
        format_duration(duration_seconds),
        block_hours
    );

    // Time info
    let current_time = now.format("%H:%M");
    let start_time = block_start_floor.format("%H:%M");
    let end_time = block_end.format("%H:%M");
    let time_info = format!("{BRIGHT_WHITE}{}{RESET} {BRIGHT_GREEN}({} to {}){RESET}",
        current_time,
        start_time,
        end_time
    );

    format!("{} {} {} {} {}", label, bar, pct_str, duration_str, time_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::StdinData;
    use crate::transcript::SessionStats;
    use chrono::Local;

    #[test]
    fn test_render_line3_with_session() {
        let now = Local::now();
        let block_start = now - chrono::Duration::hours(1);

        let stats = SessionStats {
            total_input: 10000,
            total_output: 5000,
            message_count: 10,
            block_count: 1,
            block_start: Some(block_start),
            block_end: Some(now),
            burn_timeline: vec![],
            total_cache_creation: 0,
            total_cache_read: 0,
        };

        let ctx = DisplayContext::new(
            StdinData::default(),
            Some(stats),
            80,
            None,
            None,
        );

        let result = render(&ctx, 80);
        assert!(result.contains("Session:"));
        assert!(result.contains("%"));
    }

    #[test]
    fn test_render_line3_without_stats() {
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
    fn test_render_line3_without_block_start() {
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
}
