// Terminal width detection with fallback chain and hysteresis
use std::process::Command;
use std::os::unix::io::AsRawFd;
use std::sync::Mutex;

static LAST_WIDTH: Mutex<usize> = Mutex::new(80);
const HYSTERESIS_THRESHOLD: usize = 5;

pub fn get_terminal_width() -> usize {
    let mut width = try_tmux_width()
        .or_else(try_ioctl_width)
        .or_else(try_tput_width)
        .or_else(try_env_width)
        .unwrap_or(80);

    if let Ok(mut last) = LAST_WIDTH.lock() {
        if width.abs_diff(*last) < HYSTERESIS_THRESHOLD {
            width = *last;
        } else {
            *last = width;
        }
    }

    width
}

fn try_tmux_width() -> Option<usize> {
    Command::new("tmux")
        .args(["display-message", "-p", "#{client_width}"])
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .ok()
                .and_then(|s| s.trim().parse().ok())
        })
}

fn try_ioctl_width() -> Option<usize> {
    unsafe {
        let mut ws: libc::winsize = std::mem::zeroed();
        let stdout_fd = std::io::stdout().as_raw_fd();

        if libc::ioctl(stdout_fd, libc::TIOCGWINSZ, &mut ws) == 0 && ws.ws_col > 0 {
            Some(ws.ws_col as usize)
        } else {
            None
        }
    }
}

fn try_tput_width() -> Option<usize> {
    Command::new("tput")
        .arg("cols")
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .ok()
                .and_then(|s| s.trim().parse().ok())
        })
}

fn try_env_width() -> Option<usize> {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
}
