use super::DisplayContext;
use crate::display::format::format_cost_ref;
use crate::display::width::strip_ansi;
use crate::colors::*;
use crate::git::get_dirty_status;

pub fn render(ctx: &DisplayContext, width: usize) -> String {
    let model = ctx.stdin_data.model.as_ref().map(|m| {
        m.display_name.as_ref()
            .or(m.id.as_ref())
            .map(|n| shorten_model(n))
            .unwrap_or_default()
    });

    let git_branch = ctx.git_branch.as_deref();
    let dirty = get_dirty_status(None).unwrap_or_default();

    let dir = ctx.stdin_data.cwd.as_deref()
        .or_else(|| ctx.stdin_data.workspace.as_ref()
            .and_then(|w| w.current_dir.as_deref()))
        .and_then(|path| path.split('/').last());

    let msg_count = ctx.stats.as_ref().map(|s| s.message_count);

    let cost = ctx.stdin_data.cost.as_ref()
        .and_then(|c| c.total_cost_usd)
        .map(|c| format_cost_ref(c));

    // Progressive fit - 6 levels
    let parts = vec![
        build_full(&model, git_branch, &dirty, dir, msg_count, cost.as_deref()),
        build_without_cost(&model, git_branch, &dirty, dir, msg_count),
        build_minimal(&model, git_branch, &dirty, msg_count),
        build_core(&model, git_branch, &dirty),
        build_model_only(&model),
        build_super_compact(&model),
    ];

    for part in parts {
        if strip_ansi(&part).len() <= width {
            return part;
        }
    }

    String::new()
}

fn shorten_model(name: &str) -> String {
    name.replace("Claude ", "")
        .replace(" 4.5", " 4")
        .replace(" 3.5", " 3")
}

fn build_full(
    model: &Option<String>,
    git_branch: Option<&str>,
    dirty: &str,
    dir: Option<&str>,
    msg_count: Option<usize>,
    cost: Option<&str>,
) -> String {
    let mut parts = vec![];

    if let Some(m) = model {
        parts.push(format!("{BRIGHT_YELLOW}[{}]{RESET}", m));
    }

    if let Some(branch) = git_branch {
        let dirty_display = if !dirty.is_empty() {
            format!(" {BRIGHT_YELLOW}{}{RESET}", dirty)
        } else {
            String::new()
        };
        parts.push(format!("🌿 {BRIGHT_GREEN}{}{RESET}{}", branch, dirty_display));
    }

    if let Some(d) = dir {
        parts.push(format!("📁 {BRIGHT_CYAN}{}{RESET}", d));
    }

    if let Some(count) = msg_count {
        parts.push(format!("💬 {BRIGHT_CYAN}{}{RESET}", count));
    }

    if let Some(c) = cost {
        let cost_val = c.trim_start_matches('$').parse::<f64>().unwrap_or(0.0);
        let color = if cost_val > 10.0 { BRIGHT_YELLOW } else { BRIGHT_WHITE };
        parts.push(format!("💰 {}{}{RESET}", color, c));
    }

    parts.join(" | ")
}

fn build_without_cost(
    model: &Option<String>,
    git_branch: Option<&str>,
    dirty: &str,
    dir: Option<&str>,
    msg_count: Option<usize>,
) -> String {
    let mut parts = vec![];

    if let Some(m) = model {
        parts.push(format!("{BRIGHT_YELLOW}[{}]{RESET}", m));
    }

    if let Some(branch) = git_branch {
        let dirty_display = if !dirty.is_empty() {
            format!(" {BRIGHT_YELLOW}{}{RESET}", dirty)
        } else {
            String::new()
        };
        parts.push(format!("🌿 {BRIGHT_GREEN}{}{RESET}{}", branch, dirty_display));
    }

    if let Some(d) = dir {
        parts.push(format!("📁 {BRIGHT_CYAN}{}{RESET}", d));
    }

    if let Some(count) = msg_count {
        parts.push(format!("💬 {BRIGHT_CYAN}{}{RESET}", count));
    }

    parts.join(" | ")
}

fn build_minimal(
    model: &Option<String>,
    git_branch: Option<&str>,
    dirty: &str,
    msg_count: Option<usize>,
) -> String {
    let mut parts = vec![];

    if let Some(m) = model {
        parts.push(format!("{BRIGHT_YELLOW}[{}]{RESET}", m));
    }

    if let Some(branch) = git_branch {
        let dirty_display = if !dirty.is_empty() {
            format!(" {BRIGHT_YELLOW}{}{RESET}", dirty)
        } else {
            String::new()
        };
        parts.push(format!("🌿 {BRIGHT_GREEN}{}{RESET}{}", branch, dirty_display));
    }

    if let Some(count) = msg_count {
        parts.push(format!("💬 {BRIGHT_CYAN}{}{RESET}", count));
    }

    parts.join(" | ")
}

fn build_core(
    model: &Option<String>,
    git_branch: Option<&str>,
    dirty: &str,
) -> String {
    let mut parts = vec![];

    if let Some(m) = model {
        parts.push(format!("{BRIGHT_YELLOW}[{}]{RESET}", m));
    }

    if let Some(branch) = git_branch {
        let dirty_display = if !dirty.is_empty() {
            format!(" {BRIGHT_YELLOW}{}{RESET}", dirty)
        } else {
            String::new()
        };
        parts.push(format!("🌿 {BRIGHT_GREEN}{}{RESET}{}", branch, dirty_display));
    }

    parts.join(" | ")
}

fn build_model_only(model: &Option<String>) -> String {
    if let Some(m) = model {
        format!("{BRIGHT_YELLOW}[{}]{RESET}", m)
    } else {
        String::new()
    }
}

fn build_super_compact(model: &Option<String>) -> String {
    if let Some(m) = model {
        let first_char = m.chars().next().unwrap_or('M');
        format!("{BRIGHT_YELLOW}[{}]{RESET}", first_char)
    } else {
        String::new()
    }
}
