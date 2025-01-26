use serde::{Deserialize, Serialize};
use anyhow::{Error, Result};

use crate::resources::Resources;


#[derive(Clone, Debug, Serialize, Deserialize)]
struct InputMessage {
    id: String,
    image: String,
    min_resources: Resources,
    max_resources: Resources,
    time_limit: usize,
    args: Vec<String>,
    priority: String
}

impl InputMessage {
    pub fn from_string(input: &str) -> Result<Self> {
        serde_json::from_str(input)
            .map_err(|e| Error::msg(format!("Failed to parse InputMessage from string: {}", e)))
    }
}
