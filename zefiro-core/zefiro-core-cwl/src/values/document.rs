use crate::values::types::CwlValueType;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    ops::Deref,
};

/// Represents a collection of CWL input and output values as key-value pairs
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CwlValues {
    #[serde(flatten)]
    values: HashMap<String, CwlValueType>,
}

impl Deref for CwlValues {
    type Target = HashMap<String, CwlValueType>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl CwlValues {
    /// Deserializes YAML `file` containing CWL values into CwlValues structure.
    ///
    /// ```
    /// use zefiro_core_cwl::values::document::CwlValues;
    /// let yaml_file = "examples/cwl/clt-step-values.yml";
    /// let values = CwlValues::from_path(yaml_file).expect("Failed to deserialize CWL values document");
    /// ```
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let reader = BufReader::new(
            File::open(path)
                .map_err(|e| Error::msg(format!("Failed to open file '{}': {}", path, e)))?,
        );

        serde_yaml::from_reader(reader).map_err(|e| {
            Error::msg(format!(
                "Failed to deserialize CWL values from '{}'; {}",
                path, e
            ))
        })
    }

    /// Deserializes YAML `string` containing CWL values into CwlValues structure.
    ///
    /// ```
    /// use zefiro_core_cwl::values::document::CwlValues;
    ///
    /// let yaml_input = r#"
    /// in_file:
    ///     class: File
    ///     location: 's3://bucket/path/to/input.txt'
    /// out_file: 'output.txt'
    /// "#;
    ///
    /// let values = CwlValues::from_string(yaml_input).expect("Failed to deserialize CWL values document");
    /// ```
    pub fn from_string(yaml_input: &str) -> Result<Self, Error> {
        serde_yaml::from_str(yaml_input).map_err(|e| {
            Error::msg(format!(
                "Failed to deserialize CWL values from string: {}",
                e
            ))
        })
    }

    /// Deserializes CwlValues structure into `string`.
    pub fn to_string(&self) -> Result<String, Error> {
        serde_yaml::to_string(self)
            .map_err(|e| Error::msg(format!("Failed to dserialize CWL values to string: {}", e)))
    }

    /// Serializes CwlValues structure and writes it into `file`.
    ///
    /// ```
    /// use zefiro_core_cwl::values::document::CwlValues;
    /// use std::io::BufWriter;
    ///
    /// let yaml_input = r#"
    /// in_file:
    ///     class: File
    ///     location: 's3://bucket/path/to/input.txt'
    /// out_file: 'output.txt'
    /// "#;
    ///
    /// let values = CwlValues::from_string(yaml_input).expect("Failed to serialize CWL values document");
    /// let mut tmpfile = tempfile::tempfile().unwrap();
    /// let mut writer = BufWriter::new(tmpfile);
    /// values.to_yaml(writer);
    /// ```
    pub fn to_yaml<W: Write>(&self, writer: W) -> Result<()> {
        serde_yaml::to_writer(writer, self).map_err(Into::into)
    }

    pub fn to_json(&self) -> Result<serde_json::Value, Error> {
        Ok(serde_json::to_value(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::io::BufWriter;

    #[rstest]
    #[case("examples/cwl/clt-step-values.yml")]
    fn test_cwlvalues_from_path(#[case] file_path: &str) {
        CwlValues::from_path(file_path).expect("Failed to deserialize CWL values document");
    }

    #[rstest]
    #[case("examples/cwl/clt-step-values.yml")]
    fn test_cwlvalues_to_yaml(#[case] file_path: &str) {
        let values = CwlValues::from_path(file_path).expect("Failed to deserialize CWL values");
        let temp_file = tempfile::NamedTempFile::new().unwrap();

        // Write values to temp file
        let writer = BufWriter::new(File::create(temp_file.path()).unwrap());
        values.to_yaml(writer).expect("Failed to write YAML");
        // Read and parse written content
        let written_values = CwlValues::from_path(temp_file.path().to_str().unwrap())
            .expect("Failed to read written YAML");

        assert_eq!(
            serde_yaml::to_value(&values).unwrap(),
            serde_yaml::to_value(&written_values).unwrap()
        );
    }
}
