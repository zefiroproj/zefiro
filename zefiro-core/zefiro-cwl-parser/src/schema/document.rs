use crate::schema::{
    command_line_tool::CommandLineTool,
    requirements::SUPPORTED_CWL_VERSIONS,
    types::{CLT_CWL_CLASS, WF_CWL_CLASS},
    workflow::Workflow,
};
use anyhow::{bail, ensure, Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};
use std::{
    fs::File,
    io::{BufReader, Write},
    str::FromStr,
};

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
    /// let yaml_file = "examples/cwl/clt-step-schema.yml";
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
            SUPPORTED_CWL_VERSIONS.contains(&version),
            "Unsupported CWL version: {version}"
        );

        match value.get("class").and_then(Value::as_str) {
            Some(CLT_CWL_CLASS) => Ok(Self::CommandLineTool(serde_yaml::from_value(value)?)),
            Some(WF_CWL_CLASS) => Ok(Self::Workflow(serde_yaml::from_value(value)?)),
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
    /// let yaml_file = "examples/cwl/clt-step-schema.yml";
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
    use std::io::{Error, ErrorKind, Write};

    #[rstest]
    #[case("examples/cwl/clt-step-schema.yml")]
    #[case("examples/cwl/wf-step-schema.yml")]
    fn test_cwlschema_from_path(#[case] file_path: &str) {
        CwlSchema::from_path(file_path).expect("Failed to deserialize CWL schema document");
    }

    #[rstest]
    #[case("examples/cwl/clt-step-schema.yml")]
    #[case("examples/cwl/wf-step-schema.yml")]
    fn test_cwlschema_to_yaml(#[case] file_path: &str) {
        let values = CwlSchema::from_path(file_path).expect("Failed to deserialize CWL schema");
        let temp_file = tempfile::NamedTempFile::new().unwrap();

        // Write values to temp file
        let writer = BufWriter::new(File::create(temp_file.path()).unwrap());
        values.to_yaml(writer).expect("Failed to write YAML");
        // Read and parse written content
        let written_values = CwlSchema::from_path(temp_file.path().to_str().unwrap())
            .expect("Failed to read written YAML");

        assert_eq!(
            serde_yaml::to_value(&values).unwrap(),
            serde_yaml::to_value(&written_values).unwrap()
        );
    }

    struct FailingWriter;
    impl Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(Error::new(ErrorKind::Other, "Simulated write error"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_clt_to_yaml_write_error() {
        let schema = CwlSchema::CommandLineTool(CommandLineTool::default());
        assert!(schema.to_yaml(FailingWriter).is_err());
    }

    #[test]
    fn test_wf_to_yaml_write_error() {
        let schema = CwlSchema::Workflow(Workflow::default());
        assert!(schema.to_yaml(FailingWriter).is_err());
    }
}
