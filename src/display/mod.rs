// Display module
// Wave 0: Structure + stubs
// Wave 1: agent-display implements width/format/progress
// Wave 2: agent-formatter implements line1-4, full/compact/tight/minimal/agent

pub mod width;
pub mod format;
pub mod progress;
pub mod line1;
pub mod line2;
pub mod line3;
pub mod line4;
pub mod full;
pub mod compact;
pub mod tight;
pub mod minimal;
pub mod agent;

use crate::input::StdinData;
use crate::transcript::SessionStats;

#[derive(Debug)]
pub struct DisplayContext {
    pub stdin_data: StdinData,
    pub stats: Option<SessionStats>,
    pub terminal_width: usize,
    pub git_branch: Option<String>,
    pub git_dirty: Option<String>,
}

impl DisplayContext {
    pub fn new(
        stdin_data: StdinData,
        stats: Option<SessionStats>,
        terminal_width: usize,
        git_branch: Option<String>,
        git_dirty: Option<String>,
    ) -> Self {
        Self {
            stdin_data,
            stats,
            terminal_width,
            git_branch,
            git_dirty,
        }
    }
}
