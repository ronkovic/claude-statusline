// Git branch detection with .git/HEAD fast path and git status fallback
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_branch(cwd: Option<&str>) -> Option<String> {
    let cwd = cwd.unwrap_or(".");

    read_git_head(cwd).or_else(|| read_git_status(cwd))
}

pub fn get_dirty_status(cwd: Option<&str>) -> Option<String> {
    let cwd = cwd.unwrap_or(".");

    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(cwd)
        .output()
        .ok()?;

    if !output.status.success() || output.stdout.is_empty() {
        None
    } else {
        Some("*".to_string())
    }
}

fn read_git_head(cwd: &str) -> Option<String> {
    let git_head = Path::new(cwd).join(".git/HEAD");
    let content = fs::read_to_string(git_head).ok()?;

    content
        .strip_prefix("ref: refs/heads/")
        .map(|branch| branch.trim().to_string())
}

fn read_git_status(cwd: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["status", "--porcelain", "--branch"])
        .current_dir(cwd)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    for line in stdout.lines() {
        if let Some(branch_line) = line.strip_prefix("## ") {
            let branch = branch_line
                .split("...")
                .next()?
                .split_whitespace()
                .next()?;
            return Some(branch.to_string());
        }
    }

    None
}
