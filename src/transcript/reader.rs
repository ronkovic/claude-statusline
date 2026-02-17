// BufReader with "usage" pre-filter
// Wave 1: agent-transcript implementation

use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_transcript_lines(path: &Path) -> crate::error::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::with_capacity(64 * 1024, file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // Pre-filter: only keep lines containing "usage"
        if line.contains("\"usage\"") {
            lines.push(line);
        }
    }
    Ok(lines)
}
