use super::DisplayContext;
use crate::colors::*;

pub fn render(ctx: &DisplayContext, _width: usize) -> String {
    if let Some(stats) = &ctx.stats {
        let block_info = format!("Block {}/{}", 1, stats.block_count);

        if let (Some(start), Some(end)) = (&stats.block_start, &stats.block_end) {
            let time_range = format!(
                "{} - {}",
                start.format("%H:%M"),
                end.format("%H:%M")
            );
            format!("{} {}", c(CYAN, &block_info), c(DIM, &time_range))
        } else {
            c(CYAN, &block_info)
        }
    } else {
        String::new()
    }
}
