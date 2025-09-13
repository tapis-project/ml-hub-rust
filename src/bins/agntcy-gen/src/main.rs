use std::fs;
use std::path::PathBuf;
use typify::TypeSpace;

fn main() {
    let schema_path = PathBuf::from("schemas/agent.schema.json");
    let schema_str = fs::read_to_string(schema_path).expect("failed to read schema");
    let schema: schemars::schema::RootSchema =
        serde_json::from_str(&schema_str).expect("invalid JSON schema");

    let mut typespace = TypeSpace::default();
    typespace.add_root_schema(schema).expect("failed to add schema");

    // Print the generated code once
    println!("{}", typespace.to_stream().to_string());
}