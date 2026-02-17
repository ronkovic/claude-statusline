use super::DisplayContext;
use crate::colors::*;

pub fn render(ctx: &DisplayContext) -> String {
    if let Some(agent) = &ctx.stdin_data.agent {
        if let Some(name) = &agent.name {
            format!("👤 {}", c(CYAN, name))
        } else {
            "👤 agent".to_string()
        }
    } else {
        String::new()
    }
}
