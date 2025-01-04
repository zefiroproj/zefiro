use crate::schema::requirements::CommandLineToolRequirement;
use crate::schema::types::{Any, CwlSchemaType, Documentation};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This defines the schema of the CWL Command Line Tool Description document.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandLineTool {
    pub cwl_version: String,
    pub class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<Documentation>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub inputs: Vec<CommandInputParameter>,
    pub outputs: Vec<CommandOutputParameter>,
    pub requirements: Vec<CommandLineToolRequirement>,
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
