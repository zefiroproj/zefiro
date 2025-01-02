use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value as YValue;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Any {
    Any(YValue),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum CwlSchemaType {
    Null,
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    File(String),
    Directory(String),
    Map(HashMap<String, Self>),
    Array(Vec<Self>)
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
