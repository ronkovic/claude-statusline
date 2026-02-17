use super::DisplayContext;
use crate::colors::*;

pub fn render(ctx: &DisplayContext) -> String {
    ctx.stdin_data.model.as_deref()
        .and_then(|m| m.split('-').next())
        .map(|m| c(BOLD, m))
        .unwrap_or_else(|| "?".to_string())
}
