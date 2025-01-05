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
    /// Deserializes YAML `file` containing CWL values into CwlSchema structure.
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

    /// Deserializes YAML `string` containing CWL values into CwlValues structure.
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

    /// Serializes CwlSchema structure into `string`.
    pub fn to_string(&self) -> Result<String> {
        serde_yaml::to_string(self).map_err(Into::into)
    }

    /// Serializes CwlSchema structure and writes it into `file`.
    /// ```
    /// use zefiro_cwl_parser::schema::document::CwlSchema;
    /// use std::fs::File;
    /// use std::io::BufWriter;
    ///
    /// let yaml_file = "examples/data/clt-step-schema.yml";
    /// let schema = CwlSchema::from_path(yaml_file).expect("Failed to serialize CWL schema document");
    /// let mut tmpfile = tempfile::tempfile().unwrap();
    /// let mut writer = BufWriter::new(tmpfile);
    /// schema.to_yaml(writer);
    /// ```
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
    use std::io::BufWriter;
    use std::io::Read;

    #[rstest]
    #[case("examples/data/clt-step-schema.yml")]
    #[case("examples/data/wf-step-schema.yml")]
    fn test_parse_correct_schema(#[case] file_path: &str) {
        CwlSchema::from_path(file_path).expect("Failed to deserialize CWL schema document");
    }

    #[rstest]
    #[case("examples/data/clt-step-schema.yml", tempfile::NamedTempFile::new().unwrap())]
    #[case("examples/data/wf-step-schema.yml", tempfile::NamedTempFile::new().unwrap())]
    fn test_save_schema_to_yaml(
        #[case] file_path: &str,
        #[case] temp_file: tempfile::NamedTempFile,
    ) {
        let schema =
            CwlSchema::from_path(file_path).expect("Failed to deserialize CWL schema document");
        let output_path = temp_file.path().to_path_buf();

        // Write the schema to the temporary file
        {
            let writer = BufWriter::new(File::create(&output_path).expect("Failed to create file"));
            schema
                .to_yaml(writer)
                .expect("Failed to serialize schema to YAML");
        }

        // Verify the written content by reading it back
        let written_content = {
            let mut reader =
                BufReader::new(File::open(&output_path).expect("Failed to open temporary file"));
            let mut content = String::new();
            reader
                .read_to_string(&mut content)
                .expect("Failed to read written content");
            content
        };

        // Parse both as CwlSchema and compare the resulting structures instead of raw YAML
        let original_schema =
            CwlSchema::from_path(file_path).expect("Failed to parse original file");
        let written_schema =
            CwlSchema::from_string(&written_content).expect("Failed to parse written content");

        assert_eq!(
            serde_yaml::to_value(&original_schema).unwrap(),
            serde_yaml::to_value(&written_schema).unwrap(),
            "Serialized content doesn't match original"
        );
    }
}
