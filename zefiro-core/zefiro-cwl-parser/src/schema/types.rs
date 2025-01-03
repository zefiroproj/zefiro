use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value as YValue;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Any {
    Any(YValue),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CwlSchemaType {
    /// Represents any value in field `type`
    ///
    /// Example:
    ///
    /// type: boolean
    /// ...
    Any(String),

    /// Represents an array type
    ///
    /// Example:
    ///
    /// - null
    /// - type: array
    ///   items: File
    Array(Vec<Self>),

    /// Represents a map type
    ///
    /// Example:
    ///
    /// type: array
    /// items: string
    Map(HashMap<String, Self>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Documentation {
    SingleLine(String),
    MultiLine(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Format {
    Format(String),
    Formats(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Scatter {
    Parameter(String),
    Parameters(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Source {
    SingleSource(String),
    MultiSources(Vec<String>),
}
