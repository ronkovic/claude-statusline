use super::DisplayContext;
use crate::colors::*;

pub fn render(ctx: &DisplayContext) -> String {
    if ctx.stdin_data.has_agent_teammate == Some(true) {
        let teammates: Vec<_> = ctx.stdin_data.agent_teammates.iter()
            .map(|t| {
                let status_color = match t.status.as_deref() {
                    Some("idle") => DIM,
                    Some("busy") => YELLOW,
                    _ => WHITE,
                };
                c(status_color, &t.name)
            })
            .collect();

        format!("👥 {}", teammates.join(" "))
    } else {
        String::new()
    }
}
