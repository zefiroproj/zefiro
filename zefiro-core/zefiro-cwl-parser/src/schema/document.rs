use crate::schema::{command_line_tool::CommandLineTool, workflow::Workflow};
use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};
use std::{
    fs::File, io::{BufReader, Write}, str::FromStr
};

const SUPPORTED_VERSIONS: &[&str] = &["v1.2"];


/// Represents a CWL Schema which can be either a CommandLineTool or a Workflow
/// 
/// # Variants
/// 
/// * `CommandLineTool` - Represents a single command line tool with its inputs and outputs
/// * `Workflow` - Represents a series of connected steps forming a complete workflow
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CwlSchema {
    CommandLineTool(CommandLineTool),
    Workflow(Workflow),
}

impl CwlSchema {
    pub fn from_path(path: &str) -> Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        Self::from_yaml(serde_yaml::from_reader(reader)?)
    }

    pub fn from_yaml(value: Value) -> Result<Self> {
        let version = value
            .get("cwlVersion")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Failed to determine CWL specification version."))?;
        ensure!(SUPPORTED_VERSIONS.contains(&version), "Unsupported CWL version: {version}");

        match value.get("class").and_then(Value::as_str) {
            Some("CommandLineTool") => Ok(Self::CommandLineTool(serde_yaml::from_value(value)?)),
            Some("Workflow") => Ok(Self::Workflow(serde_yaml::from_value(value)?)),
            Some(class) => bail!("Unsupported CWL document class: {class}"),
            None => bail!("Failed to determine CWL document class."),
        }
    }

    pub fn to_string(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(Into::into)
    }

    pub fn to_yaml<W: Write>(&self, writer: W) -> Result<()> {
        serde_yaml::to_writer(writer, self).map_err(Into::into)
    }
}

impl FromStr for CwlSchema {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_yaml(serde_yaml::from_str(s)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("examples/data/clt-step-schema.yml")]
    #[case("examples/data/wf-step-schema.yml")]
    fn test_parse_correct_schema(#[case] file_path: &str) {
        CwlSchema::from_path(file_path).expect("Failed to deserialize CWL schema document");
    }
}