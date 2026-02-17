// JSONL transcript file finder with 6-hour mtime filter
// Wave 1: agent-transcript implementation

use std::path::PathBuf;
use std::time::{SystemTime, Duration};

pub fn find_recent_transcripts() -> crate::error::Result<Vec<PathBuf>> {
    let base_dir = dirs::home_dir()
        .ok_or_else(|| crate::error::Error::Other("No home dir".into()))?
        .join(".claude/projects");

    // Handle missing directory gracefully
    if !base_dir.exists() {
        return Ok(vec![]);
    }

    let cutoff = SystemTime::now() - Duration::from_secs(6 * 3600);

    let mut files = Vec::new();
    for entry in std::fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
            if let Ok(meta) = path.metadata() {
                if let Ok(modified) = meta.modified() {
                    if modified >= cutoff {
                        files.push(path);
                    }
                }
            }
        }
    }
    Ok(files)
}
