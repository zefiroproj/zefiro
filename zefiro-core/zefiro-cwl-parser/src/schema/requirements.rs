use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub const SUPPORTED_CWL_VERSIONS: [&str; 1] = ["v1.2"];

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
    WorkReuse(WorkReuse),
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
    #[serde(default = "ResourceRequirement::cores_min")]
    pub cores_min: u32,

    #[serde(default = "ResourceRequirement::ram_min")]
    pub ram_min: u32,

    #[serde(default = "ResourceRequirement::tmpdir_min")]
    pub tmpdir_min: u32,

    #[serde(default = "ResourceRequirement::outdir_min")]
    pub outdir_min: u32,
}

impl ResourceRequirement {
    const fn cores_min() -> u32 {
        CPU_NUM_DEFAULT
    }
    const fn ram_min() -> u32 {
        RAM_SIZE_IN_MB_DEFAULT
    }
    const fn tmpdir_min() -> u32 {
        TMPDIR_MIN_IN_MB_DEFAULT
    }
    const fn outdir_min() -> u32 {
        OUTDIR_MIN_IN_MB_DEFAULT
    }
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
    pub timelimit: Timelimit,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Timelimit {
    Seconds(u32),
    Expression(String),
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
