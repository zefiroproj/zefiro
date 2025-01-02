use crate::values::types::CwlValueType;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    collections::HashMap, fs::File, io::{BufReader, Write}, ops::Deref
};

/// Represents a collection of CWL input/output values as key-value pairs
/// 
/// # Fields
/// 
/// * `values` - A map of parameter names to their corresponding CWL values.
///             The values are flattened during serialization/deserialization
///             to allow for a more natural YAML representation
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CwlValues {
    #[serde(flatten)]
    values: HashMap<String, CwlValueType>
}

impl Deref for CwlValues {
    type Target = HashMap<String, CwlValueType>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl CwlValues {
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let reader = BufReader::new(File::open(path).map_err(|e| {
            Error::msg(format!("Failed to open file '{}': {}", path, e))
        })?);
        
        serde_yaml::from_reader(reader).map_err(|e| {
            Error::msg(format!("Failed to parse CWL values from '{}'; {}", path, e))
        })
    }

    pub fn to_string(&self) -> Result<String, Error> {
        Ok(serde_yaml::to_string(self)?)
    }

    pub fn to_yaml<W: Write>(&self, writer: W) -> Result<()> {
        serde_yaml::to_writer(writer, self).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("examples/data/clt-step-values.yml")]
    fn test_parse_correct_values(#[case] file_path: &str) {
        CwlValues::from_path(file_path).expect("Failed to deserialize CWL values document");
    }
}