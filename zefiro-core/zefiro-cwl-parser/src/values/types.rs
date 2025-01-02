use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CwlInputClass {
    File(String),
    Directory(String),
}

/// Represents a `File` object in CWL
/// see: https://www.commonwl.org/v1.2/CommandLineTool.html#File
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CwlFile {
    pub class: CwlInputClass,
    pub location: String,
}

impl CwlFile {
    pub fn get_location(&self) -> String {
        self.location.clone()
    }
}

/// Represents a `Directory` object in CWL
/// see: https://www.commonwl.org/v1.2/CommandLineTool.html#Directory
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CwlDirectory {
    pub class: CwlInputClass,
    pub location: String,
}

impl CwlDirectory {
    pub fn get_location(&self) -> String {
        self.location.clone()
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CwlValueType {
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    CwlFile(CwlFile),
    CwlDirectory(CwlDirectory),
    Array(Vec<Self>)
}