// ANSI color constants and NO_COLOR handling
use std::sync::OnceLock;

static COLORS_DISABLED: OnceLock<bool> = OnceLock::new();

pub fn colors_enabled() -> bool {
    !*COLORS_DISABLED.get_or_init(|| {
        std::env::var("NO_COLOR").is_ok() || std::env::var("STATUSLINE_NO_COLOR").is_ok()
    })
}

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const BG_RED: &str = "\x1b[41m";
pub const BLINK: &str = "\x1b[5m";

pub fn c(code: &str, text: &str) -> String {
    if colors_enabled() {
        format!("{}{}{}", code, text, RESET)
    } else {
        text.to_string()
    }
}
