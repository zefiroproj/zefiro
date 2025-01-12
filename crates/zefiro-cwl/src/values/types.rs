use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::io;
use std::path::Path;

/// Represents a `File` object in CWL
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CwlFile {
    /// Full path to the file, e.g., "/path/to/file.txt".
    pub location: String,

    /// Basename of the file, e.g., "file.txt".
    #[serde(default)]
    pub basename: Option<String>,

    /// Basename without the file extension, e.g., "file".
    #[serde(default)]
    pub nameroot: Option<String>,

    /// File extension, e.g., "txt".
    #[serde(default)]
    pub nameext: Option<String>,

    /// File size in bytes, e.g., 1024.
    #[serde(default)]
    pub size: Option<u64>,

    /// SHA-1 checksum of the file, e.g., "c63b83369243849f80049b2726dcc8db0b18d03e".
    #[serde(default)]
    pub checksum: Option<String>,
}

impl CwlFile {
    pub fn location(&self) -> String {
        self.location.clone()
    }

    pub fn calculate_checksum(path: &str) -> io::Result<String> {
        let file = fs::File::open(path)?;
        let mut reader = io::BufReader::new(file);
        let mut hasher = Sha1::new();
        io::copy(&mut reader, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn extract_path_info<F, T>(path: &str, provided: Option<T>, extractor: F) -> Option<T>
    where
        F: Fn(&Path) -> Option<T>,
    {
        provided.or_else(|| extractor(Path::new(path)))
    }

    pub fn basename(path: &str, provided_basename: Option<String>) -> Option<String> {
        Self::extract_path_info(path, provided_basename, |p| {
            p.file_name()
                .and_then(|name| name.to_str().map(String::from))
        })
    }

    pub fn nameroot(path: &str, provided_nameroot: Option<String>) -> Option<String> {
        Self::extract_path_info(path, provided_nameroot, |p| {
            p.file_stem()
                .and_then(|stem| stem.to_str().map(String::from))
        })
    }

    pub fn nameext(path: &str, provided_nameext: Option<String>) -> Option<String> {
        Self::extract_path_info(path, provided_nameext, |p| {
            p.extension().and_then(|ext| ext.to_str().map(String::from))
        })
    }

    pub fn size(path: &str, provided_size: Option<u64>) -> io::Result<Option<u64>> {
        Ok(provided_size.or_else(|| fs::metadata(path).ok().map(|m| m.len())))
    }

    pub fn checksum(path: &str, provided_checksum: Option<String>) -> Option<String> {
        provided_checksum.or_else(|| Self::calculate_checksum(path).ok())
    }
}

/// Represents a `Directory` object in CWL
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CwlDirectory {
    pub location: String,
}

impl CwlDirectory {
    pub fn location(&self) -> &str {
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
