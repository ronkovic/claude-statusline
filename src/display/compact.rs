use super::DisplayContext;
use super::line1;
use crate::display::format::format_tokens;
use crate::colors::*;

pub fn render(ctx: &DisplayContext) -> String {
    let width = ctx.terminal_width;
    let line1 = line1::render(ctx, width);

    if let Some(stats) = &ctx.stats {
        let tokens = format!(
            "{}↑ {}↓",
            format_tokens(stats.total_input),
            format_tokens(stats.total_output)
        );
        format!("{}\n{}", line1, c(DIM, &tokens))
    } else {
        line1
    }
}
