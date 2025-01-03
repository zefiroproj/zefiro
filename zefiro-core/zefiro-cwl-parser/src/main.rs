use zefiro_cwl_parser::values::{document::CwlValues, types::CwlValueType};
use zefiro_cwl_parser::schema::document::CwlSchema;

fn main() {
    let file_path = "examples/data/clt-step-values.yml";
    let values = CwlValues::from_path(file_path).expect("Failed to deserialize CWL values document");
    for (key, value) in values.iter() {
        match value {
            CwlValueType::File(value) => println!("{} {:?} {:?}", key, value, value.get_location()),
            _ => println!("{} {:?}", key, value)
        }
    }

    let file_path = "examples/data/wf-step-schema.yml";
    let schema = CwlSchema::from_path(file_path).expect("Failed to deserialize CWL values document");
    println!("{:?}", schema);
    // for (key, value) in schema.iter() {
    //     println!("{:?}", value);
    // }
}