use serde::{Deserialize, Serialize};

/// Represents a `File` object in CWL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub location: String,
}

impl File {
    pub fn get_location(&self) -> String {
        self.location.clone()
    }
}

/// Represents a `Directory` object in CWL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Directory {
    pub location: String,
}

impl Directory {
    pub fn get_location(&self) -> &str {
        &self.location
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "class", rename_all = "PascalCase")]
pub enum Path {
    File(File),
    Directory(Directory),
}

/// CWL value types with tagged enum for `File` and `Directory`
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CwlValueType {
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Path(Path),
    Array(Vec<Self>),
}
