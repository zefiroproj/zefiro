use anyhow::{Context, Error};
use deno_core::{serde_json, serde_v8, v8, JsRuntime};

pub struct JsEngine {
    runtime: JsRuntime,
}

impl JsEngine {
    /// Creates a new `JsEngine` with the given `inputs` JSON string.
    pub fn new(inputs: &str, outputs: &str, self_obj: &str) -> Result<Self, Error> {
        let mut runtime = JsRuntime::new(Default::default());
        let init_script = format!(
            "const inputs = {};const outputs = {};const self = {};",
            inputs, outputs, self_obj
        );

        runtime
            .execute_script("<init>", init_script)
            .context("Error initializing JavaScript context")?;

        Ok(Self { runtime })
    }

    /// Executes the given JavaScript `script` and returns the result as an `f64`.
    pub fn run(&mut self, script: String) -> Result<String, Error> {
        let result = self
            .runtime
            .execute_script("<eval>", script)
            .context("Error executing JavaScript expression")?;

        let scope = &mut self.runtime.handle_scope();
        let local = v8::Local::new(scope, result);
        let result_json: serde_json::Value =
            serde_v8::from_v8(scope, local).context("Error deserializing result")?;

        Ok(result_json.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    #[case(
        json!({
            "in_fastq": {
                "location": "/path/to/input.fastq",
                "size": 1024 * 1024 * 512,
            }
        }).to_string(),
        json!({
            "out_fastq": {
                "location": "/path/to/output.fastq",
            }
        }).to_string(),
        json!({
            "self": {
                "location": "/path/to/output.fastq",
            }
        }).to_string(),
        r#"const fastq_size = inputs.in_fastq.size / (1024 * 1024);
        fastq_size * 2;"#,
        "1024",
    )]
    #[case(
        json!({"output_location_subdir": "output/"}).to_string(),
        json!({"out_fastq": [{
            "location": "/path/to/output.fastq",
            "basename": "output.fastq",
            "nameroot": "output",
            "nameext": "fastq",
        }]}).to_string(),
        json!({"self": [{
            "location": "/path/to/output.fastq",
            "basename": "output.fastq",
            "nameroot": "output",
            "nameext": "fastq",
        }]}).to_string(),
        r#"
        self[0].location = inputs.output_location_subdir + self[0].nameroot + '.fq';
        return self[0]
        "#,
        json!({
            "location": "output/output.fq",
            "basename": "output.fastq",
            "nameroot": "output",
            "nameext": "fastq"
        }).to_string(),
    )]
    fn test_jsexecutor_run(
        #[case] inputs: String,
        #[case] outputs: String,
        #[case] self_obj: String,
        #[case] js_script: String,
        #[case] expected: String,
    ) {
        let mut executor = JsEngine::new(&inputs, &outputs, &self_obj)
            .expect("Failed to deserialize CWL schema document");
        let result = executor
            .run(js_script)
            .expect("JavaScript execution failed")
            .to_string();
        assert_eq!(result, expected);
    }
}
