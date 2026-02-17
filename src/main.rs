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
    let stats = transcript::load_and_analyze()?;
    let terminal_width = terminal::get_terminal_width();
    let git_branch = git::get_branch(stdin_data.cwd.as_deref());
    let git_dirty = git::get_dirty_status(stdin_data.cwd.as_deref());

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

fn select_display_mode(ctx: &display::DisplayContext) -> String {
    // Agent teams mode takes priority
    if ctx.stdin_data.has_agent_teammate == Some(true) {
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
