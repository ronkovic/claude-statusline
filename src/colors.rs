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
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BLINK: &str = "\x1b[5m";
pub const BRIGHT_CYAN: &str = "\x1b[1;96m";
pub const BRIGHT_GREEN: &str = "\x1b[1;92m";
pub const BRIGHT_YELLOW: &str = "\x1b[1;93m";
pub const BRIGHT_RED: &str = "\x1b[1;91m";
pub const BRIGHT_WHITE: &str = "\x1b[1;97m";
pub const BRIGHT_BLUE: &str = "\x1b[1;94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[1;95m";
pub const LIGHT_GRAY: &str = "\x1b[37m";

pub fn c(code: &str, text: &str) -> String {
    if colors_enabled() {
        format!("{}{}{}", code, text, RESET)
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bright_colors_defined() {
        assert_eq!(BRIGHT_CYAN, "\x1b[1;96m");
        assert_eq!(BRIGHT_GREEN, "\x1b[1;92m");
        assert_eq!(BRIGHT_YELLOW, "\x1b[1;93m");
        assert_eq!(BRIGHT_RED, "\x1b[1;91m");
        assert_eq!(BRIGHT_WHITE, "\x1b[1;97m");
        assert_eq!(BRIGHT_BLUE, "\x1b[1;94m");
        assert_eq!(BRIGHT_MAGENTA, "\x1b[1;95m");
    }

    #[test]
    fn test_bg_colors_defined() {
        assert_eq!(BG_RED, "\x1b[41m");
        assert_eq!(BG_YELLOW, "\x1b[43m");
    }

    #[test]
    fn test_light_gray_defined() {
        assert_eq!(LIGHT_GRAY, "\x1b[37m");
    }

    #[test]
    fn test_color_function() {
        let result = c(RED, "test");
        assert!(result.contains("test"));
    }
}
