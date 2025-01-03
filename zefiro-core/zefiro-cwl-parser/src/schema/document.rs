use crate::schema::{command_line_tool::CommandLineTool, workflow::Workflow};
use anyhow::{bail, ensure, Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};
use std::{
    fs::File,
    io::{BufReader, Write},
    str::FromStr,
};

const SUPPORTED_VERSIONS: &[&str] = &["v1.2"];

/// Represents a CWL Schema which can be either a CommandLineTool or a Workflow
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CwlSchema {
    CommandLineTool(CommandLineTool),
    Workflow(Workflow),
}

impl CwlSchema {
    /// Serializes YAML `file` containing CWL values into CwlSchema structure.
    ///
    /// ```
    /// use zefiro_cwl_parser::schema::document::CwlSchema;
    ///
    /// let yaml_file = "examples/data/clt-step-schema.yml";
    ///
    /// let values = CwlSchema::from_path(yaml_file).expect("Failed to deserialize CWL values document");
    /// ```
    pub fn from_path(path: &str) -> Result<Self> {
        let reader = BufReader::new(File::open(path)?);
        Self::from_yaml(serde_yaml::from_reader(reader)?)
    }

    /// Deserializes a YAML Value into a CwlSchema instance.
    pub fn from_yaml(value: Value) -> Result<Self> {
        let version = value
            .get("cwlVersion")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow::anyhow!("Failed to determine CWL specification version."))?;
        ensure!(
            SUPPORTED_VERSIONS.contains(&version),
            "Unsupported CWL version: {version}"
        );

        match value.get("class").and_then(Value::as_str) {
            Some("CommandLineTool") => Ok(Self::CommandLineTool(serde_yaml::from_value(value)?)),
            Some("Workflow") => Ok(Self::Workflow(serde_yaml::from_value(value)?)),
            Some(class) => bail!("Unsupported CWL document class: {class}"),
            None => bail!("Failed to determine CWL document class."),
        }
    }

    /// Serializes YAML `string` containing CWL values into CwlValues structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_yaml::Value;
    /// use zefiro_cwl_parser::schema::document::CwlSchema;
    ///
    /// let yaml_str = r#"
    /// cwlVersion: v1.2
    /// class: CommandLineTool
    /// id: step
    /// inputs:
    ///   - id: in_file
    ///     type: File
    ///     inputBinding:
    ///       prefix: --in-file
    ///   - id: out_file
    ///     type: string
    ///     default: "output.txt"
    ///     inputBinding:
    ///       prefix: --out-file
    ///   - id: output_location_subdir
    ///     type: string
    ///     default: output/
    /// outputs:
    ///   - id: out_file
    ///     type: File
    ///     outputBinding:
    ///       glob: $(inputs.out_file)
    ///       outputEval: ${self[0].location += inputs.output_location_subdir; return self[0]}
    /// requirements:
    ///     - class: DockerRequirement
    ///       dockerPull: step-image-uri:1.0
    ///     - class: InlineJavascriptRequirement
    /// "#;
    ///
    /// let schema = CwlSchema::from_string(yaml_str).expect("Failed to parse CWL document");
    /// ```
    pub fn from_string(yaml_input: &str) -> Result<Self, Error> {
        serde_yaml::from_str(yaml_input)
            .map_err(|e| Error::msg(format!("Failed to parse CWL schema from string: {}", e)))
    }

    /// Deserializes CwlValues structure into `string`.
    pub fn to_string(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(Into::into)
    }

    /// Deserializes CwlValues structure and writes it into `file`.
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
