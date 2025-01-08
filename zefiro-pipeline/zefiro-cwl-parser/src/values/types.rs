use serde::{Deserialize, Deserializer, Serialize};
use sha1::{Digest, Sha1};
use std::fs;
use std::io;
use std::path::Path;

/// Represents a `File` object in CWL
#[derive(Clone, Debug, Serialize, Default)]
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

    fn calculate_checksum(path: &str) -> io::Result<String> {
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha1::new();
        io::copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn extract_path_info<F, T>(path: &str, provided: Option<T>, extractor: F) -> Option<T>
    where
        F: Fn(&Path) -> Option<T>,
    {
        provided.or_else(|| extractor(Path::new(path)))
    }

    fn basename(path: &str, provided_basename: Option<String>) -> Option<String> {
        Self::extract_path_info(path, provided_basename, |p| {
            p.file_name()
                .and_then(|name| name.to_str().map(String::from))
        })
    }

    fn nameroot(path: &str, provided_nameroot: Option<String>) -> Option<String> {
        Self::extract_path_info(path, provided_nameroot, |p| {
            p.file_stem()
                .and_then(|stem| stem.to_str().map(String::from))
        })
    }

    fn nameext(path: &str, provided_nameext: Option<String>) -> Option<String> {
        Self::extract_path_info(path, provided_nameext, |p| {
            p.extension().and_then(|ext| ext.to_str().map(String::from))
        })
    }

    fn size(path: &str, provided_size: Option<u64>) -> Option<u64> {
        provided_size.or_else(|| fs::metadata(path).ok().map(|m| m.len()))
    }

    fn checksum(path: &str, provided_checksum: Option<String>) -> Option<String> {
        provided_checksum.or_else(|| Self::calculate_checksum(path).ok())
    }
}

impl<'de> Deserialize<'de> for CwlFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            location: String,
            #[serde(default)]
            basename: Option<String>,
            #[serde(default)]
            nameroot: Option<String>,
            #[serde(default)]
            nameext: Option<String>,
            #[serde(default)]
            size: Option<u64>,
            #[serde(default)]
            checksum: Option<String>,
        }

        let helper = Helper::deserialize(deserializer)?;
        let path = &helper.location;

        Ok(Self {
            location: helper.location.clone(),
            basename: Self::basename(path, helper.basename),
            nameroot: Self::nameroot(path, helper.nameroot),
            nameext: Self::nameext(path, helper.nameext),
            size: Self::size(path, helper.size),
            checksum: Self::checksum(path, helper.checksum),
        })
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
