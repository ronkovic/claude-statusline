use super::DisplayContext;
use super::{line1, line2, line3, line4};
use crate::display::width::strip_ansi;

pub fn render(ctx: &DisplayContext) -> String {
    let width = ctx.terminal_width;
    let mut lines = vec![];

    // Line 1: そのまま使用
    let l1 = line1::render(ctx, width);
    if !l1.is_empty() {
        lines.push(l1);
    }

    // Line 2: Compactラベル短縮
    let l2_full = line2::render(ctx, width);
    let l2 = l2_full.replace("Compact:", "C:");
    if !l2.is_empty() {
        lines.push(l2);
    }

    // Line 3: Sessionラベル短縮
    let l3_full = line3::render(ctx, width);
    let l3 = l3_full.replace("Session:", "S:");
    if !l3.is_empty() {
        lines.push(l3);
    }

    // Line 4: Burnラベル短縮
    let l4_full = line4::render(ctx, width);
    let l4 = l4_full.replace("Burn:   ", "B:");
    if !l4.is_empty() {
        lines.push(l4);
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::{StdinData, ModelInfo, ContextWindow, CurrentUsage, CostInfo};
    use crate::transcript::SessionStats;
    use chrono::Local;

    #[test]
    fn test_compact_labels() {
        let now = Local::now();
        let block_start = now - chrono::Duration::hours(2);

        let stats = SessionStats {
            total_input: 140000,
            total_output: 20000,
            message_count: 100,
            block_count: 1,
            block_start: Some(block_start),
            block_end: None,
            burn_timeline: vec![100; 20],
            total_cache_creation: 5000,
            total_cache_read: 2000,
            duration_seconds: 7200,
        };

        let stdin_data = StdinData {
            model: Some(ModelInfo {
                id: Some("claude-sonnet-4-5".to_string()),
                display_name: Some("Sonnet 4.5".to_string()),
            }),
            context_window: Some(ContextWindow {
                total_input_tokens: Some(140000),
                total_output_tokens: Some(20000),
                used_percentage: Some(89),
                context_window_size: Some(200000),
                remaining_percentage: Some(11),
                current_usage: Some(CurrentUsage {
                    input_tokens: Some(8500),
                    output_tokens: Some(1200),
                    cache_creation_input_tokens: Some(5000),
                    cache_read_input_tokens: Some(2000),
                }),
            }),
            cost: Some(CostInfo {
                total_cost_usd: Some(0.044),
                total_duration_ms: None,
                total_api_duration_ms: None,
            }),
            cwd: None,
            session_id: None,
            transcript_path: None,
            workspace: None,
            agent: None,
            message_count: None,
            session_stats: None,
            burn_timeline: None,
        };

        let ctx = DisplayContext::new(
            stdin_data,
            Some(stats),
            80,
            None,
            None,
        );

        let output = render(&ctx);
        let lines: Vec<&str> = output.split('\n').collect();

        // Line 2 should start with "C:"
        assert!(lines.get(1).map(|l| strip_ansi(l).starts_with("C:")).unwrap_or(false),
            "Line 2 should start with 'C:' but got: {:?}", lines.get(1));

        // Line 3 should start with "S:"
        assert!(lines.get(2).map(|l| strip_ansi(l).starts_with("S:")).unwrap_or(false),
            "Line 3 should start with 'S:' but got: {:?}", lines.get(2));

        // Line 4 should start with "B:"
        assert!(lines.get(3).map(|l| strip_ansi(l).starts_with("B:")).unwrap_or(false),
            "Line 4 should start with 'B:' but got: {:?}", lines.get(3));
    }
}
