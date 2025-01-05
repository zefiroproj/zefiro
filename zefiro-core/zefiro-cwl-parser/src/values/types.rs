use serde::{Deserialize, Deserializer, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::io;
use std::path::Path;

/// Represents a `File` object in CWL
#[derive(Clone, Debug, Serialize, Default)]
pub struct CwlFile {
    pub location: String,
    #[serde(default)]
    pub basename: Option<String>,
    #[serde(default)]
    pub nameroot: Option<String>,
    #[serde(default)]
    pub nameext: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub checksum: Option<String>,
}

impl CwlFile {
    pub fn location(&self) -> String {
        self.location.clone()
    }

    fn calculate_checksum(path: &str) -> io::Result<String> {
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha1::new();
        io::copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn size(path: &str, provided_size: Option<u64>) -> Option<u64> {
        provided_size.or_else(|| fs::metadata(path).ok().map(|m| m.len()))
    }

    fn checksum(path: &str, provided_checksum: Option<String>) -> Option<String> {
        provided_checksum.or_else(|| CwlFile::calculate_checksum(path).ok())
    }

    fn basename(path: &str) -> Option<String> {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }

    fn nameroot(path: &str) -> Option<String> {
        Path::new(path)
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }

    fn nameext(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }
}

impl<'de> Deserialize<'de> for CwlFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct FileHelper {
            location: String,
            size: Option<u64>,
            checksum: Option<String>,
        }

        let helper = FileHelper::deserialize(deserializer)?;
        let path = &helper.location;

        Ok(CwlFile {
            location: helper.location.clone(),
            basename: CwlFile::basename(path),
            nameroot: CwlFile::nameroot(path),
            nameext: CwlFile::nameext(path),
            size: CwlFile::size(path, helper.size),
            checksum: CwlFile::checksum(path, helper.checksum),
        })
    }
}

/// Represents a `Directory` object in CWL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CwlDirectory {
    pub location: String,
}

impl CwlDirectory {
    pub fn get_location(&self) -> &str {
        &self.location
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "class", rename_all = "PascalCase")]
pub enum CwlPath {
    File(CwlFile),
    Directory(CwlDirectory),
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
    Path(CwlPath),
    Array(Vec<Self>),
}
