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

    // Output directory - works from both workspace root and dfns-gen directory
    let output_dir = if Path::new("dfns-client/src").exists() {
        // Running from workspace root
        Path::new("dfns-client/src")
    } else if Path::new("../dfns-client/src").exists() {
        // Running from dfns-gen directory
        Path::new("../dfns-client/src")
    } else {
        panic!("Cannot find dfns-client/src directory. Run from workspace root or dfns-gen directory.");
    };

    // Generate Rust code with NEW generator (better naming!)
    println!("\nGenerating Rust code from OpenAPI schemas (with improved naming)...");
    let mut generator = NewGenerator::new(openapi.clone());
    let generated_code = generator.generate();

    let generated_path = output_dir.join("generated.rs");
    fs::write(&generated_path, generated_code).expect("Failed to write generated.rs");
    println!("✅ Generated Rust types written to {}", generated_path.display());
    println!("   → Using nested modules with clean naming!");

    // Save IR for endpoint generator
    let ir = generator.into_ir();
    let ir_path = output_dir.join("dfns_gen_ir.json");
    ir.save_to_file(&ir_path).expect("Failed to write IR");
    println!("✅ IR saved to {}", ir_path.display());
    println!("   → {} schemas, {} endpoints", ir.schema_locations.len(), ir.endpoints.len());

    // Generate API endpoints (now using IR)
    println!("\nGenerating API endpoints...");
    let endpoint_generator = EndpointGenerator::new_with_ir(openapi, ir);
    let api_code = endpoint_generator.generate();

    let api_path = output_dir.join("generated_api.rs");
    fs::write(&api_path, api_code).expect("Failed to write generated_api.rs");
    println!("✅ Generated API server written to {}", api_path.display());

    println!("\n🎉 Generation complete! The dfns-client crate is ready to use.");
    println!("   → Types: {}", generated_path.display());
    println!("   → API Server: {}", api_path.display());
    println!("   → Client: ../dfns-client/src/client/");
}
