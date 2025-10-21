// Demo of the new IR-based code generation pipeline
// This shows the complete flow: OpenAPI → IR → AST → Formatted Code

use dfns_gen::codegen::{ir_to_ast, openapi_to_ir};
use dfns_gen::ir::*;

#[tokio::main]
async fn main() {
    println!("=== New IR-Based Pipeline Demo ===\n");

    // Demo 1: Manual IR construction
    demo_manual_ir();

    // Demo 2: OpenAPI to IR (small example)
    demo_openapi_to_ir().await;
}

fn demo_manual_ir() {
    println!("1. Manual IR → Code Generation:\n");

    // Build a struct manually using IR
    let wallet_struct = StructDef::new("Wallet")
        .with_doc("Represents a cryptocurrency wallet")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_field(
            FieldDef::new("id", RustType::String)
                .with_doc("Unique wallet identifier")
                .with_serde_rename("walletId")
        )
        .with_field(
            FieldDef::new("network", RustType::String)
                .with_doc("Blockchain network")
                .required()
        )
        .with_field(
            FieldDef::new("balance", RustType::F64)
                .with_doc("Current balance")
                .optional()
        )
        .with_field(
            FieldDef::new("tags", RustType::Vec(Box::new(RustType::String)))
                .optional()
        );

    // Build an enum
    let status_enum = EnumDef::new("WalletStatus")
        .with_doc("Wallet status")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_variant(VariantDef::unit("Active").as_default())
        .with_variant(VariantDef::unit("Inactive"))
        .with_variant(VariantDef::tuple("Suspended", RustType::String));

    // Create complete output
    let output = CodeGenOutput::new()
        .with_type(TypeDef::Struct(wallet_struct))
        .with_type(TypeDef::Enum(status_enum));

    // Generate code
    let generated_code = ir_to_ast::generate_code_output(&output);

    println!("Generated Code:");
    println!("{}", "=".repeat(70));
    println!("{}", generated_code);
    println!("{}", "=".repeat(70));
    println!();
}

async fn demo_openapi_to_ir() {
    println!("2. OpenAPI → IR → Code (Simple Example):\n");

    // Create a minimal OpenAPI spec manually
    let openapi_yaml = r#"
openapi: 3.0.0
info:
  title: Demo API
  version: 1.0.0
components:
  schemas:
    User:
      type: object
      description: A user account
      required:
        - id
        - email
      properties:
        id:
          type: string
          description: User ID
        email:
          type: string
          description: User email
        name:
          type: string
          description: Display name
        age:
          type: integer
          format: int32

    Status:
      type: string
      description: Account status
      enum:
        - active
        - inactive
        - suspended
"#;

    // Parse OpenAPI spec
    match oas3::from_yaml(openapi_yaml) {
        Ok(spec) => {
            println!("✓ Parsed OpenAPI spec");

            // Convert to IR
            let mut converter = openapi_to_ir::OpenApiToIr::new(spec);
            let output = converter.convert();

            println!("✓ Converted to IR:");
            println!("  - {} top-level types", output.types.len());
            println!("  - {} modules", output.modules.len());

            // Generate code
            let generated_code = ir_to_ast::generate_code_output(&output);

            println!("\n✓ Generated Code:");
            println!("{}", "=".repeat(70));
            println!("{}", generated_code);
            println!("{}", "=".repeat(70));
        }
        Err(e) => {
            eprintln!("Error parsing OpenAPI: {}", e);
        }
    }
}
