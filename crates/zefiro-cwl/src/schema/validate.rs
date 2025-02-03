use crate::schema::requirements::MINIMAL_CWL_VERSION;
use validator::ValidationError;

use regex::Regex;

use super::types::{CLT_CWL_CLASS, WF_CWL_CLASS};

const CWL_VERSION_PATTERN: &str = r"^v([\d.]+)";

pub fn validate_cwl_version(cwl_version: &str) -> Result<(), ValidationError> {
    let re = Regex::new(CWL_VERSION_PATTERN).expect("Failed to parse CWL version regex pattern.");

    let provided_version = re
        .captures(cwl_version)
        .and_then(|caps| caps.get(0))
        .ok_or_else(|| ValidationError::new("Invalid CWL version format"))?
        .as_str()
        .parse::<f32>()
        .map_err(|_| ValidationError::new("CWL version must be a floating-point number"))?;

    if provided_version < MINIMAL_CWL_VERSION {
        return Err(ValidationError::new(Box::leak(
            format!(
                "Minimal supported CWL version is {}. But provided: {}",
                MINIMAL_CWL_VERSION, provided_version
            )
            .into_boxed_str(),
        )));
    }

    Ok(())
}

pub fn validate_cwl_class(cwl_class: &str) -> Result<(), ValidationError> {
    match cwl_class {
        WF_CWL_CLASS => Ok(()),
        CLT_CWL_CLASS => Ok(()),
        _ => Err(ValidationError::new(Box::leak(
            format!(
                "Expected {} or {} CWL class. But provided: {}",
                WF_CWL_CLASS, CLT_CWL_CLASS, cwl_class
            )
            .into_boxed_str(),
        ))),
    }
}
