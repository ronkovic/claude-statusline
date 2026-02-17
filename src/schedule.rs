// Calendar schedule via `gog` CLI
// Wave 1: agent-schedule implementation

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScheduleEvent {
    pub summary: String,
    pub start: String,
    pub end: String,
}

pub fn fetch_schedule() -> crate::error::Result<Vec<ScheduleEvent>> {
    // Stub: agent-schedule implements (gog CLI + cache)
    Ok(vec![])
}
