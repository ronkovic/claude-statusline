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
    // Wave 3: Full implementation
    eprintln!("cc-statusline stub");
    Ok(())
}
