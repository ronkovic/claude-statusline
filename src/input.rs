// stdin JSON input parsing
use serde::Deserialize;
use std::io::{self, Read};

#[derive(Debug, Deserialize, Default)]
pub struct StdinData {
    pub model: Option<ModelInfo>,
    pub cwd: Option<String>,
    pub session_id: Option<String>,
    pub transcript_path: Option<String>,
    pub context_window: Option<ContextWindow>,
    pub cost: Option<CostInfo>,
    pub workspace: Option<WorkspaceInfo>,
    pub agent: Option<AgentInfo>,
    pub message_count: Option<usize>,
    pub session_stats: Option<SessionStatsInput>,
    pub burn_timeline: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContextWindow {
    pub total_input_tokens: Option<u64>,
    pub total_output_tokens: Option<u64>,
    pub context_window_size: Option<u64>,
    pub used_percentage: Option<u64>,
    pub remaining_percentage: Option<u64>,
    pub current_usage: Option<CurrentUsage>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentUsage {
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
    pub cache_creation_input_tokens: Option<u64>,
    pub cache_read_input_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CostInfo {
    pub total_cost_usd: Option<f64>,
    pub total_duration_ms: Option<u64>,
    pub total_api_duration_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct WorkspaceInfo {
    pub current_dir: Option<String>,
    pub project_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgentInfo {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SessionStatsInput {
    pub message_count: Option<usize>,
    pub total_input_tokens: Option<u64>,
    pub total_output_tokens: Option<u64>,
    pub block_count: Option<usize>,
    pub block_start: Option<String>,
    pub block_end: Option<String>,
    pub burn_timeline: Option<Vec<u64>>,
    pub total_cache_creation: Option<u64>,
    pub total_cache_read: Option<u64>,
    pub duration_seconds: Option<i64>,
}

impl StdinData {
    pub fn read() -> crate::error::Result<Self> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        let data: StdinData = serde_json::from_str(&buffer)?;
        Ok(data)
    }

    pub fn model_id(&self) -> &str {
        self.model
            .as_ref()
            .and_then(|m| m.id.as_deref())
            .unwrap_or("unknown")
    }

    pub fn model_display(&self) -> &str {
        self.model
            .as_ref()
            .and_then(|m| m.display_name.as_deref())
            .or_else(|| self.model.as_ref().and_then(|m| m.id.as_deref()))
            .unwrap_or("unknown")
    }
}
