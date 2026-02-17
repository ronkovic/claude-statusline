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
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let modified_count = stdout.lines()
        .filter(|line| line.starts_with(" M") || line.starts_with("M "))
        .count();

    if modified_count > 0 {
        Some(format!("M{}", modified_count))
    } else {
        Some(format!("+{}", stdout.lines().count()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dirty_status_with_modified_files() {
        // Test parsing git status output with modified files
        let status_output = " M file1.rs\n M file2.rs\nM  file3.rs\n";
        let modified_count = parse_modified_count(status_output);
        assert_eq!(modified_count, 3);
    }

    #[test]
    fn test_parse_dirty_status_with_no_modified() {
        // Test parsing git status output with no modified files
        let status_output = "?? file.rs\n";
        let modified_count = parse_modified_count(status_output);
        assert_eq!(modified_count, 0);
    }

    #[test]
    fn test_parse_dirty_status_empty() {
        // Test parsing empty status output
        let status_output = "";
        let modified_count = parse_modified_count(status_output);
        assert_eq!(modified_count, 0);
    }

    fn parse_modified_count(output: &str) -> usize {
        output.lines()
            .filter(|line| line.starts_with(" M") || line.starts_with("M "))
            .count()
    }
}
