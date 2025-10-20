mod codegen;

// #[allow(dead_code)]
// pub mod generated;

use codegen::{EndpointGenerator, Generator};
use std::fs;

#[tokio::main]
async fn main() {
    println!("Fetching OpenAPI spec from Dfns...");
    let r = reqwest::get("https://docs.dfns.co/openapi.yaml")
        .await
        .expect("Failed to fetch OpenAPI spec");
    let yaml_content = r.text().await.expect("Failed to read response");
    fs::write("openapi.yml", &yaml_content).expect("Failed to write openapi.json");

    // Parse and save as JSON
    let openapi = oas3::from_yaml(&yaml_content).expect("Failed to parse OpenAPI YAML");
    let openapi_json = oas3::to_json(&openapi).expect("Failed to convert to JSON");

    fs::write("openapi.json", &openapi_json).expect("Failed to write openapi.json");
    println!("Saved OpenAPI spec to openapi.json");

    // Generate Rust code
    println!("Generating Rust code from OpenAPI schemas...");
    let mut generator = Generator::new(openapi.clone());
    let generated_code = generator.generate();

    fs::write("src/generated.rs", generated_code).expect("Failed to write generated.rs");
    println!("Generated Rust code written to src/generated.rs");

    // Generate API endpoints
    println!("Generating API endpoints...");
    let endpoint_generator = EndpointGenerator::new(openapi);
    let api_code = endpoint_generator.generate();

    fs::write("src/generated_api.rs", api_code).expect("Failed to write generated_api.rs");
    println!("Generated API endpoints written to src/generated_api.rs");
}
