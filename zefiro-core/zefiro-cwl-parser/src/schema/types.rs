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
    // Here can be any type:
    // Null, Boolean, Int, Long, Float, Double, String, File, Directory
    // array, string[], File[], Directory[]
    Any(String),
    
    Array(Vec<Self>),
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
