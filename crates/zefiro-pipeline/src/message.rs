use serde::{Deserialize, Serialize};
use anyhow::{Error, Result};

use zefiro_job::priority::JobPriority;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub schema: String,
    pub values: String,
    pub priority: JobPriority
}

impl Message {
    pub fn from_string(input: &str) -> Result<Self> {
        serde_json::from_str(input)
            .map_err(|e| Error::msg(format!("Failed to parse Message from string: {}", e)))
    }
}
