// Use the library's modules instead of declaring them locally
use dfns_gen::codegen::{EndpointGenerator, NewGenerator};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    println!("Fetching OpenAPI spec from Dfns...");
    let r = reqwest::get("https://docs.dfns.co/openapi.yaml")
        .await
        .expect("Failed to fetch OpenAPI spec");
    let yaml_content = r.text().await.expect("Failed to read response");
    fs::write("openapi.yml", &yaml_content).expect("Failed to write openapi.yml");

    // Parse and save as JSON
    let openapi = oas3::from_yaml(&yaml_content).expect("Failed to parse OpenAPI YAML");
    let openapi_json = oas3::to_json(&openapi).expect("Failed to convert to JSON");

    fs::write("openapi.json", &openapi_json).expect("Failed to write openapi.json");
    println!("Saved OpenAPI spec to openapi.json");

    // Output directory is dfns-client crate
    let output_dir = Path::new("../dfns-client/src");

    // Generate Rust code with NEW generator (better naming!)
    println!("\nGenerating Rust code from OpenAPI schemas (with improved naming)...");
    let mut generator = NewGenerator::new(openapi.clone());
    let generated_code = generator.generate();

    let generated_path = output_dir.join("generated.rs");
    fs::write(&generated_path, generated_code).expect("Failed to write generated.rs");
    println!("✅ Generated Rust types written to {}", generated_path.display());
    println!("   → Using nested modules with clean naming!");

    // Generate API endpoints
    println!("\nGenerating API endpoints...");
    let endpoint_generator = EndpointGenerator::new(openapi);
    let api_code = endpoint_generator.generate();

    let api_path = output_dir.join("generated_api.rs");
    fs::write(&api_path, api_code).expect("Failed to write generated_api.rs");
    println!("✅ Generated API server written to {}", api_path.display());

    println!("\n🎉 Generation complete! The dfns-client crate is ready to use.");
    println!("   → Types: {}", generated_path.display());
    println!("   → API Server: {}", api_path.display());
    println!("   → Client: ../dfns-client/src/client/");
}
