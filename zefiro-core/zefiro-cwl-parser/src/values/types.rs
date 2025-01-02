use serde::{Deserialize, Serialize};

/// Represents a `File` object in CWL
/// see: https://www.commonwl.org/v1.2/CommandLineTool.html#File
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub class: String,
    pub location: String,
}

impl File {
    pub fn get_location(&self) -> String {
        self.location.clone()
    }
}

/// Represents a `Directory` object in CWL
/// see: https://www.commonwl.org/v1.2/CommandLineTool.html#Directory
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Directory {
    pub class: String,
    pub location: String,
}

impl Directory {
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
    File(File),
    Directory(Directory),
    Array(Vec<Self>)
}