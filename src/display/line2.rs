use super::DisplayContext;
use crate::display::format::format_tokens;
use crate::colors::*;

pub fn render(ctx: &DisplayContext, _width: usize) -> String {
    // Prefer context_window from stdin (Claude Code provides it)
    if let Some(context_window) = &ctx.stdin_data.context_window {
        let in_tok = context_window.total_input_tokens.unwrap_or(0);
        let out_tok = context_window.total_output_tokens.unwrap_or(0);
        let used_pct = context_window.used_percentage.unwrap_or(0);

        let warning = if used_pct >= 85 {
            c(&format!("{}{}", BG_RED, BLINK), " ⚠ ")
        } else {
            String::new()
        };

        format!(
            "{}in:{} out:{}",
            warning,
            c(GREEN, &format_tokens(in_tok)),
            c(YELLOW, &format_tokens(out_tok))
        )
    } else if let Some(stats) = &ctx.stats {
        // Fallback: use transcript stats
        let in_tok = format_tokens(stats.total_input);
        let out_tok = format_tokens(stats.total_output);

        format!("in:{} out:{}", c(GREEN, &in_tok), c(YELLOW, &out_tok))
    } else {
        String::new()
    }
}
