// cc-statusline: Claude Code status line tool in Rust
// Wave 0: Scaffold with all module declarations

mod cli;
mod input;
mod error;
mod config;
mod tokens;
mod terminal;
mod colors;
mod git;
mod schedule;
mod display;
mod transcript;

use error::Result;

fn main() -> Result<()> {
    let mode = cli::parse_args()?;

    match mode {
        cli::Mode::Schedule => show_schedule()?,
        cli::Mode::Show => show_status()?,
    }

    Ok(())
}

fn show_schedule() -> Result<()> {
    let events = schedule::fetch_schedule()?;

    if events.is_empty() {
        println!("No upcoming events");
    } else {
        for event in events {
            println!("{}: {} - {}", event.summary, event.start, event.end);
        }
    }

    Ok(())
}

fn show_status() -> Result<()> {
    let stdin_data = input::StdinData::read()?;

    // Determine transcript path: prefer explicit transcript_path, fallback to finder
    let transcript_path = stdin_data.transcript_path.as_deref();

    // Load stats from transcript or JSON
    let stats = if let Some(session_stats) = &stdin_data.session_stats {
        // Convert SessionStatsInput to SessionStats
        convert_session_stats(session_stats)
    } else {
        transcript::load_and_analyze(transcript_path)?
    };

    let terminal_width = terminal::get_terminal_width();

    // Determine working directory: prefer cwd, fallback to workspace.current_dir
    let working_dir = stdin_data.cwd.as_deref()
        .or_else(|| stdin_data.workspace.as_ref().and_then(|w| w.current_dir.as_deref()));

    let git_branch = git::get_branch(working_dir);
    let git_dirty = git::get_dirty_status(working_dir);

    let ctx = display::DisplayContext::new(
        stdin_data,
        stats,
        terminal_width,
        git_branch,
        git_dirty,
    );

    let output = select_display_mode(&ctx);
    print!("{}", output);

    Ok(())
}

fn convert_session_stats(input: &input::SessionStatsInput) -> Option<transcript::SessionStats> {
    use chrono::{DateTime, Local};

    let block_start = input.block_start.as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Local));

    let block_end = input.block_end.as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Local));

    Some(transcript::SessionStats {
        message_count: input.message_count.unwrap_or(0),
        total_input: input.total_input_tokens.unwrap_or(0),
        total_output: input.total_output_tokens.unwrap_or(0),
        block_count: input.block_count.unwrap_or(0),
        block_start,
        block_end,
        burn_timeline: input.burn_timeline.clone().unwrap_or_default(),
        total_cache_creation: input.total_cache_creation.unwrap_or(0),
        total_cache_read: input.total_cache_read.unwrap_or(0),
        duration_seconds: input.duration_seconds.unwrap_or(0),
    })
}

fn select_display_mode(ctx: &display::DisplayContext) -> String {
    // Agent teams mode takes priority
    if ctx.stdin_data.agent.is_some() {
        return display::agent::render(ctx);
    }

    // Width-based mode selection
    if ctx.terminal_width >= config::FULL_WIDTH_MIN {
        display::full::render(ctx)
    } else if ctx.terminal_width >= config::COMPACT_WIDTH_MIN {
        display::compact::render(ctx)
    } else if ctx.terminal_width >= config::TIGHT_WIDTH_MIN {
        display::tight::render(ctx)
    } else {
        display::minimal::render(ctx)
    }
}
