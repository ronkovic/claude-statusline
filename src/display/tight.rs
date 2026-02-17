use super::DisplayContext;
use crate::colors::*;

pub fn render(ctx: &DisplayContext) -> String {
    let model = ctx.stdin_data.model.as_deref()
        .and_then(|m| m.split('-').next())
        .unwrap_or("?");

    if let Some(stats) = &ctx.stats {
        format!("{} {}msg", c(BOLD, model), stats.message_count)
    } else {
        c(BOLD, model)
    }
}
