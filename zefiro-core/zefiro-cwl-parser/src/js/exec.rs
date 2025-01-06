use anyhow::{Context, Error};
use deno_core::{serde_json, serde_v8, v8, JsRuntime};

pub struct JsEngine {
    runtime: JsRuntime,
}

impl JsEngine {
    /// Creates a new `JsExecutor` with the given `inputs` JSON string.
    pub fn new(inputs: &str) -> Result<Self, Error> {
        let mut runtime = JsRuntime::new(Default::default());
        let init_script = format!("const inputs = {};", inputs);

        runtime
            .execute_script("<init>", init_script)
            .context("Error initializing JavaScript context")?;

        Ok(Self { runtime })
    }

    /// Executes the given JavaScript `script` and returns the result as an `f64`.
    pub fn run(&mut self, script: String) -> Result<f64, Error> {
        let result = self
            .runtime
            .execute_script("<eval>", script)
            .context("Error executing JavaScript expression")?;
        
        let scope = &mut self.runtime.handle_scope();
        let local = v8::Local::new(scope, result);
        let result_json: serde_json::Value = serde_v8::from_v8(scope, local)
            .context("Error deserializing result")?;
        result_json.as_f64().ok_or_else(|| {
            Error::msg(format!("Result is not a number: {:?}", result_json))
        })
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
            "fastq": {
                "location": "/path/to/file.txt",
                "size": 1024 * 1024 * 512,
            }
        }).to_string(),
        r#"
        const fastq = inputs.fastq.size / (1024 * 1024);
        fastq * 2;
        "#
    )]
    fn test_jsexecutor_run(#[case] inputs: String, #[case] js_script: String) {
        let mut executor = JsEngine::new(&inputs).expect("Failed to deserialize CWL schema document");
        let result = executor.run(js_script.to_string()).expect("JavaScript execution failed");
        assert_eq!(result, 1024.0);
    }
}
