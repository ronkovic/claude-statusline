use super::DisplayContext;
use crate::colors::*;

pub fn render(ctx: &DisplayContext) -> String {
    let model = ctx.stdin_data.model_display();
    let short = model.split('-').next().unwrap_or(model);
    c(BOLD, short)
}
