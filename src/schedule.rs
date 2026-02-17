// Calendar schedule via `gog` CLI
// Wave 1: agent-schedule implementation

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{SystemTime, Duration};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScheduleEvent {
    pub summary: String,
    pub start: String,
    pub end: String,
}

const CACHE_TTL_SECONDS: u64 = 300;  // 5 minutes

pub fn fetch_schedule() -> crate::error::Result<Vec<ScheduleEvent>> {
    let cache_path = get_cache_path()?;

    // Check cache
    if let Ok(cached) = read_cache(&cache_path) {
        return Ok(cached);
    }

    // Fetch via gog CLI
    let output = Command::new("gog")
        .arg("calendar")
        .arg("--json")
        .output()?;

    if !output.status.success() {
        return Ok(vec![]);  // Silently fail if gog unavailable
    }

    let events: Vec<ScheduleEvent> = serde_json::from_slice(&output.stdout)?;

    // Write cache
    write_cache(&cache_path, &events)?;

    Ok(events)
}

fn get_cache_path() -> crate::error::Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| crate::error::Error::Other("No cache dir".into()))?;
    Ok(cache_dir.join("cc-statusline-schedule.json"))
}

fn read_cache(path: &PathBuf) -> Result<Vec<ScheduleEvent>, Box<dyn std::error::Error>> {
    let meta = std::fs::metadata(path)?;
    let age = SystemTime::now().duration_since(meta.modified()?)?;

    if age < Duration::from_secs(CACHE_TTL_SECONDS) {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Err("Cache expired".into())
    }
}

fn write_cache(path: &PathBuf, events: &[ScheduleEvent]) -> crate::error::Result<()> {
    let json = serde_json::to_string(events)?;
    std::fs::write(path, json)?;
    Ok(())
}
