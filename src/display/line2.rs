use super::DisplayContext;
use crate::colors::*;
use crate::display::format::format_tokens_short;
use crate::display::progress::render_progress_bar;

pub fn render(ctx: &DisplayContext, _width: usize) -> String {
    let cw = match &ctx.stdin_data.context_window {
        Some(cw) => cw,
        None => return String::new(),
    };

    // Percentage
    let percentage = cw.used_percentage.unwrap_or_else(|| {
        let total = cw.total_input_tokens.unwrap_or(0) + cw.total_output_tokens.unwrap_or(0);
        let threshold = cw.context_window_size.unwrap_or(200_000) * 80 / 100;
        if threshold > 0 {
            ((total * 100 / threshold) as u64).min(100)
        } else {
            0
        }
    });

    // Tokens
    let compact_tokens = cw.total_input_tokens.unwrap_or(0) + cw.total_output_tokens.unwrap_or(0);
    let threshold = cw.context_window_size.unwrap_or(200_000) * 80 / 100;

    // Cache ratio
    let current = cw.current_usage.as_ref();
    let cache_read = current.and_then(|u| u.cache_read_input_tokens).unwrap_or(0);
    let cache_creation = current.and_then(|u| u.cache_creation_input_tokens).unwrap_or(0);
    let total_with_cache = compact_tokens + cache_read + cache_creation;
    let cache_ratio = if total_with_cache > 0 {
        (cache_read * 100 / total_with_cache) as u64
    } else {
        0
    };

    // Label
    let label_color = if percentage >= 85 {
        format!("{BG_RED}{BRIGHT_WHITE}\x1b[1m")
    } else {
        BRIGHT_CYAN.to_string()
    };
    let label = format!("{}Compact:{RESET}", label_color);

    // Progress bar
    let bar = render_progress_bar(percentage, 100, 20);

    // Percentage display
    let pct_color = if percentage >= 85 {
        format!("{BG_RED}{BRIGHT_WHITE}\x1b[1m")
    } else {
        BRIGHT_WHITE.to_string()
    };
    let pct_str = format!("{}[{}%]{RESET}", pct_color, percentage);

    // Tokens
    let tokens_str = format!(
        "{BRIGHT_WHITE}{}/{}{RESET}",
        format_tokens_short(compact_tokens),
        format_tokens_short(threshold)
    );

    // Cache (only if >= 50%)
    let cache_str = if cache_ratio >= 50 {
        format!(" {BRIGHT_GREEN}♻️ {}% cached{RESET}", cache_ratio)
    } else {
        String::new()
    };

    // Warning emoji (only if >= 85%)
    let warning = if percentage >= 85 {
        format!("{BRIGHT_RED}⚠️ {RESET}")
    } else {
        String::new()
    };

    format!(
        "{} {} {} {}{}{}",
        label, bar, pct_str, warning, tokens_str, cache_str
    )
}
