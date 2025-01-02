use crate::schema::types::Any;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const CPU_NUM_DEFAULT: u32 = 1;
const RAM_SIZE_IN_MB_DEFAULT: u32 = 1024;
const TMPDIR_MIN_IN_MB_DEFAULT: u32 = 1024;
const OUTDIR_MIN_IN_MB_DEFAULT: u32 = 1024;


/// Describes requirements for `Workflow`.
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "class")]
pub enum WorkflowRequirement {
    InlineJavascriptRequirement(InlineJavascriptRequirement),
    ScatterFeatureRequirement(ScatterFeatureRequirement),
}

/// Describes requirements for `CommandLineTool`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "class")]
pub enum CommandLineToolRequirement {
    DockerRequirement(DockerRequirement),
    ResourceRequirement(ResourceRequirement),
    InlineJavascriptRequirement(InlineJavascriptRequirement),
    ToolTimeLimit(ToolTimeLimit),
    WorkReuse(WorkReuse)
}

/// Specifies Docker container requirements.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#DockerRequirement
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerRequirement {
    pub docker_pull: String,
}

/// Specifies resource constraints for running the tool.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#ResourceRequirement
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirement {
    #[serde(default = "ResourceRequirement::default_cores_min")]
    pub cores_min: u32,
    
    #[serde(default = "ResourceRequirement::default_ram_min")]
    pub ram_min: u32,

    #[serde(default = "ResourceRequirement::default_tmpdir_min")]
    pub tmpdir_min: u32,

    #[serde(default = "ResourceRequirement::default_outdir_min")]
    pub outdir_min: u32,
}

impl ResourceRequirement {
    fn default_cores_min() -> u32 { CPU_NUM_DEFAULT }
    fn default_ram_min() -> u32 { RAM_SIZE_IN_MB_DEFAULT }
    fn default_tmpdir_min() -> u32 { TMPDIR_MIN_IN_MB_DEFAULT }
    fn default_outdir_min() -> u32 { OUTDIR_MIN_IN_MB_DEFAULT }
}

/// Indicates that the workflow platform must support inline Javascript expressions
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#InlineJavascriptRequirement
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineJavascriptRequirement;

/// Specifies an upper limit on the execution time of a `CommandLineTool` (in seconds).
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#ToolTimeLimit
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolTimeLimit {
    pub timelimit: Any,
}

/// Specifies that the workflow platform must support the scatter and `scatterMethod` fields of `WorkflowStep`.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#ScatterFeatureRequirement
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScatterFeatureRequirement;


/// Specifies a reusing output from past work of a `CommandLineTool`.
/// See: https://www.commonwl.org/v1.2/CommandLineTool.html#WorkReuse
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkReuse {
    pub enable_reuse: bool,
}