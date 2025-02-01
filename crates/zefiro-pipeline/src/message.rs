use serde::{Deserialize, Serialize};
use anyhow::{Error, Result};

use crate::{priority::JobPriority, resources::JobResources};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub job_id: String,
    pub image: String,
    pub min_resources: JobResources,
    pub max_resources: Option<JobResources>,
    pub time_limit: usize,
    pub args: Vec<String>,
    pub priority: JobPriority
}

impl Message {
    pub fn from_string(input: &str) -> Result<Self> {
        serde_json::from_str(input)
            .map_err(|e| Error::msg(format!("Failed to parse InputMessage from string: {}", e)))
    }
}
