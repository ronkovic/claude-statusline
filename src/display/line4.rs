use super::DisplayContext;
use crate::display::progress::render_sparkline;
use crate::display::format::format_tokens;
use crate::colors::*;

pub fn render(ctx: &DisplayContext, _width: usize) -> String {
    if let Some(stats) = &ctx.stats {
        let sparkline_width = stats.burn_timeline.len().min(20);
        let sparkline = render_sparkline(&stats.burn_timeline, sparkline_width);
        let total: u64 = stats.burn_timeline.iter().sum();
        let rate = format_tokens(total / 20);

        format!("{} {}/15min", c(YELLOW, &sparkline), c(DIM, &rate))
    } else {
        String::new()
    }
}
