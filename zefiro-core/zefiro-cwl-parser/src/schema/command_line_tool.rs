use crate::schema::requirements::{CommandLineToolRequirement, SUPPORTED_CWL_VERSIONS};
use crate::schema::types::{Any, CwlSchemaType, Documentation};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This defines the schema of the CWL Command Line Tool Description document.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommandLineTool {
    #[serde(default = "CommandLineTool::default_cwl_version")]
    pub cwl_version: String,
    #[serde(default = "CommandLineTool::default_class")]
    pub class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<Documentation>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    pub inputs: Vec<CommandInputParameter>,
    #[serde(default)]
    pub outputs: Vec<CommandOutputParameter>,
    #[serde(default)]
    pub requirements: Vec<CommandLineToolRequirement>,
}

impl CommandLineTool {
    fn default_cwl_version() -> String {
        SUPPORTED_CWL_VERSIONS[0].to_string()
    }

    fn default_class() -> String {
        "CommandLineTool".to_string()
    }
}

/// Represents an input parameter for a `CommandLineTool`.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#CommandInputParameter
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandInputParameter {
    pub id: String,

    pub r#type: CwlSchemaType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_binding: Option<CommandLineBinding>,

    pub default: Option<Any>,
}

/// Represents an output parameter for a `CommandLineTool`.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#CommandOutputParameter
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutputParameter {
    pub id: String,

    pub r#type: CwlSchemaType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_binding: Option<OutputBinding>,
}

/// Describes how to bind an input or output to the command line.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#CommandLineBinding
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandLineBinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<String>,
}

/// Describes how to find and capture output files or values from a CommandLineTool execution.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#CommandOutputBinding
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputBinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glob: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_eval: Option<String>,
}
