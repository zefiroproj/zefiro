# zefiro-cwl

A Rust library for parsing and working with Common Workflow Language (CWL) documents.

## Overview

* Supports **only some fields** of CWL **v1.2** specification (see description of structures in the code)
* Can serialize and deserialize [CommandLineTool](https://www.commonwl.org/v1.2/CommandLineTool.html) and [Workflow](https://www.commonwl.org/v1.2/Workflow.html) documents

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
zefiro-cwl = "0.0.0-pre"
```


### How to parse CWL Schema document?

```rust
use zefiro_cwl::CwlSchema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Parse from file
  let schema = CwlSchema::from_path("test_data/cwl/clt-step-schema.yml").unwrap();

  // Parse from string
  let yaml_str = r#"
  cwlVersion: v1.2
  class: CommandLineTool
  id: step
  inputs:
      - id: in_file
        type: File
        inputBinding:
          prefix: --in-file
      - id: out_file
        type: string
        default: "output.txt"
        inputBinding:
          prefix: --out-file
      - id: output_location_subdir
        type: string
        default: output/
  outputs:
      - id: out_file
        type: File
        outputBinding:
          glob: $(inputs.out_file)
          outputEval: ${self[0].location += inputs.output_location_subdir; return self[0]}
  requirements:
      - class: DockerRequirement
        dockerPull: step-image-uri:1.0
      - class: InlineJavascriptRequirement
  "#;
  let schema = CwlSchema::from_string(yaml_str).unwrap();
  
  Ok(())
}
```


### How to parse CWL Values document?

```rust
use zefiro_cwl::CwlValues;


fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Parse input values from file
  let values = CwlValues::from_path("test_data/cwl/clt-step-values.yml").unwrap();

  // Create values from string
  let yaml_input = r#"
  input_file:
      class: File
      location: 's3://bucket/input.txt'
  output_file: 'output.txt'
  "#;
  let values = CwlValues::from_string(yaml_input).unwrap();

  Ok(())
}
```

### How to execute JavaScript expressions?
