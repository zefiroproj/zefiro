use crate::schema::command_line_tool::CommandLineTool;
use crate::schema::requirements::{WorkflowRequirement, MINIMAL_CWL_VERSION};
use crate::schema::types::{Any, CwlSchemaType, Documentation, Scatter, Source, WF_CWL_CLASS};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This defines the schema of the CWL Workflow Description document.
/// See: https://www.commonwl.org/v1.2/Workflow.html
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    #[serde(default = "Workflow::default_cwl_version")]
    pub cwl_version: String,
    #[serde(default = "Workflow::default_class")]
    pub class: String,
    pub doc: Option<Documentation>,
    #[serde(default)]
    pub id: String,
    pub label: Option<String>,
    pub inputs: Vec<WorkflowInputParameter>,
    pub outputs: Vec<WorkflowOutputParameter>,
    pub steps: Vec<WorkflowStep>,
    pub requirements: Vec<WorkflowRequirement>,
}

impl Workflow {
    fn default_cwl_version() -> String {
        MINIMAL_CWL_VERSION.to_string()
    }

    fn default_class() -> String {
        WF_CWL_CLASS.to_string()
    }
}

/// Represents an input parameter for a `Workflow`.
/// See: https://www.commonwl.org/v1.2/Workflow.html#WorkflowInputParameter
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowInputParameter {
    pub r#type: CwlSchemaType,
    pub label: Option<String>,
    pub default: Option<Any>,
    pub id: Option<String>,
}

/// Represents an output parameter for a `Workflow`.
/// See: https://www.commonwl.org/v1.2/Workflow.html#WorkflowOutputParameter
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowOutputParameter {
    pub r#type: CwlSchemaType,
    pub label: Option<String>,
    pub doc: Option<Documentation>,
    pub id: Option<String>,
    pub output_source: Option<WorkflowOutputParameterOutputSource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum WorkflowOutputParameterOutputSource {
    OutputSource(String),
    OutputSourceArray(Vec<String>),
}

/// Represents a `WorkflowStep` - an executable element of a workflow.
/// See: https://www.commonwl.org/v1.2/Workflow.html#WorkflowStep
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStep {
    pub r#in: Vec<WorkflowStepInput>,
    pub out: Vec<WorkflowStepOutput>,
    pub run: CommandLineTool,
    pub id: Option<String>,
    pub label: Option<String>,
    pub doc: Option<Documentation>,
    pub scatter: Option<Scatter>,
    pub scatter_method: Option<String>,
}

/// Defines the input parameters of the workflow step (`out` section).
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepInput {
    pub id: String,
    pub source: Option<Source>,
    pub label: Option<String>,
    pub default: Option<Any>,
    pub value_from: Option<String>,
}

/// Defines the output parameters of the workflow step (`out` section).
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStepOutput {
    pub id: String,
}
