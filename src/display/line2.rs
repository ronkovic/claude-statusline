use super::DisplayContext;
use crate::display::format::format_tokens;
use crate::colors::*;

pub fn render(ctx: &DisplayContext, _width: usize) -> String {
    if let Some(stats) = &ctx.stats {
        let in_tok = format_tokens(stats.total_input);
        let out_tok = format_tokens(stats.total_output);

        let context_limit = get_context_limit(&ctx.stdin_data.model);
        let pct = stats.total_output as f64 / context_limit as f64;
        let warning = if pct >= 0.85 {
            c(&format!("{}{}", BG_RED, BLINK), " ⚠ ")
        } else {
            String::new()
        };

        format!("{}in:{} out:{}", warning, c(GREEN, &in_tok), c(YELLOW, &out_tok))
    } else {
        String::new()
    }
}

fn get_context_limit(model: &Option<String>) -> u64 {
    let model_str = model.as_deref().unwrap_or("").to_lowercase();

    if model_str.contains("opus") {
        200_000
    } else if model_str.contains("sonnet") {
        200_000
    } else if model_str.contains("haiku") {
        200_000
    } else {
        200_000
    }
}
