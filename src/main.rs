// Use the library's modules instead of declaring them locally
use dfns_gen::codegen::{EndpointGenerator, NewGenerator};
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

    // Generate Rust code with NEW generator (better naming!)
    println!("Generating Rust code from OpenAPI schemas (with improved naming)...");
    let mut generator = NewGenerator::new(openapi.clone());
    let generated_code = generator.generate();

    fs::write("src/generated.rs", generated_code).expect("Failed to write generated.rs");
    println!("✅ Generated Rust code written to src/generated.rs");
    println!("   → Using nested modules with clean naming!");

    // Generate API endpoints
    println!("Generating API endpoints...");
    let endpoint_generator = EndpointGenerator::new(openapi);
    let api_code = endpoint_generator.generate();

    fs::write("src/generated_api.rs", api_code).expect("Failed to write generated_api.rs");
    println!("✅ Generated API endpoints written to src/generated_api.rs");
}
