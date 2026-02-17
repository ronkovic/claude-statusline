// ANSI color constants and NO_COLOR handling
// Wave 1: agent-display implementation

pub fn colors_enabled() -> bool {
    // Stub: agent-display implements (NO_COLOR/STATUSLINE_NO_COLOR check)
    true
}

// ANSI codes (stub)
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
