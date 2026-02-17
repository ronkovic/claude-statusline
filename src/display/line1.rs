use super::DisplayContext;
use crate::display::{width::*, format::*};
use crate::colors::*;
use crate::tokens::{calculate_cost, TokenUsage};

pub fn render(ctx: &DisplayContext, width: usize) -> String {
    let model = ctx.stdin_data.model_display();
    let git = format_git(ctx);
    let dir = format_dir(ctx);
    let msgs = format_messages(ctx);
    let cost = format_cost_info(ctx);

    for stage in 0..6 {
        let line = build_line(model, &git, &dir, &msgs, &cost, stage);
        if display_width(&line) <= width {
            return line;
        }
    }

    truncate_to_width(&build_line(model, &git, &dir, &msgs, &cost, 5), width)
}

fn build_line(model: &str, git: &str, dir: &str, msgs: &str, cost: &str, stage: usize) -> String {
    let parts: Vec<&str> = match stage {
        0 => vec![model, git, dir, msgs, cost].into_iter().filter(|s| !s.is_empty()).collect(),
        1 => vec![shorten_model(model), git, dir, cost].into_iter().filter(|s| !s.is_empty()).collect(),
        2 => vec![shorten_model(model), git, cost].into_iter().filter(|s| !s.is_empty()).collect(),
        3 => vec![shorten_model(model), git].into_iter().filter(|s| !s.is_empty()).collect(),
        4 => vec![shorten_model(model), cost].into_iter().filter(|s| !s.is_empty()).collect(),
        _ => vec![shorten_model(model)],
    };
    parts.join(" ")
}

fn format_git(ctx: &DisplayContext) -> String {
    if let Some(branch) = &ctx.git_branch {
        let dirty = ctx.git_dirty.as_deref().unwrap_or("");
        c(CYAN, &format!("git:{}{}", branch, dirty))
    } else {
        String::new()
    }
}

fn format_dir(ctx: &DisplayContext) -> String {
    let dir = ctx.stdin_data.cwd.as_deref()
        .or_else(|| ctx.stdin_data.workspace.as_ref().and_then(|w| w.current_dir.as_deref()));

    dir.map(|cwd| {
        let short = cwd.split('/').last().unwrap_or(cwd);
        c(BLUE, short)
    }).unwrap_or_default()
}

fn format_messages(ctx: &DisplayContext) -> String {
    if let Some(stats) = &ctx.stats {
        c(GREEN, &format!("{}msg", stats.message_count))
    } else {
        String::new()
    }
}

fn format_cost_info(ctx: &DisplayContext) -> String {
    // Prefer cost from stdin (Claude Code provides it)
    if let Some(cost_info) = &ctx.stdin_data.cost {
        if let Some(cost_usd) = cost_info.total_cost_usd {
            return c(MAGENTA, &format_cost(cost_usd));
        }
    }

    // Fallback: calculate from transcript stats
    if let Some(stats) = &ctx.stats {
        let model_id = ctx.stdin_data.model_id();
        let usage = TokenUsage {
            input_tokens: stats.total_input,
            output_tokens: stats.total_output,
            cache_creation_input_tokens: None,
            cache_read_input_tokens: None,
        };
        let cost = calculate_cost(model_id, &usage);
        c(MAGENTA, &format_cost(cost))
    } else {
        String::new()
    }
}

fn shorten_model(model: &str) -> &str {
    model.split('-').next().unwrap_or(model)
}
