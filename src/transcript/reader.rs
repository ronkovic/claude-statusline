// BufReader with "usage" pre-filter
// Wave 1: agent-transcript implementation

use std::path::Path;

pub fn read_transcript_lines(_path: &Path) -> crate::error::Result<Vec<String>> {
    // Stub: BufReader(64KB) + "usage" filter
    Ok(vec![])
}
