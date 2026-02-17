// stdin JSON input parsing
use serde::Deserialize;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
pub struct StdinData {
    pub model: Option<String>,
    pub cwd: Option<String>,
    pub has_agent_teammate: Option<bool>,
    #[serde(default)]
    pub agent_teammates: Vec<AgentTeammate>,
}

#[derive(Debug, Deserialize)]
pub struct AgentTeammate {
    pub name: String,
    pub status: Option<String>,
}

impl StdinData {
    pub fn read() -> crate::error::Result<Self> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        let data: StdinData = serde_json::from_str(&buffer)?;
        Ok(data)
    }
}
