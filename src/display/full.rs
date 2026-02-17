use super::DisplayContext;
use super::{line1, line2, line3, line4};

pub fn render(ctx: &DisplayContext) -> String {
    let width = ctx.terminal_width;

    let mut lines = vec![
        line1::render(ctx, width),
        line2::render(ctx, width),
    ];

    if ctx.stats.is_some() {
        lines.push(line3::render(ctx, width));
        lines.push(line4::render(ctx, width));
    }

    lines.join("\n")
}
