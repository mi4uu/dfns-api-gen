// Example demonstrating how to use the IR (Intermediate Representation) module
// to define Rust types in a structured way

use dfns_gen::ir::*;

fn main() {
    println!("=== IR Demo: Building Type Definitions ===\n");

    // Example 1: Simple struct
    let user_struct = build_user_struct();
    println!("1. User Struct:");
    println!("{:#?}\n", user_struct);

    // Example 2: Enum with different variant types
    let status_enum = build_status_enum();
    println!("2. Status Enum:");
    println!("{:#?}\n", status_enum);

    // Example 3: Complete code generation output
    let output = build_complete_output();
    println!("3. Complete CodeGen Output:");
    println!("   - {} top-level types", output.types.len());
    println!("   - {} modules", output.modules.len());
    println!("   - {} imports", output.imports.len());

    // Example 4: Serialize IR to JSON (useful for debugging or saving intermediate state)
    println!("\n4. Serialized User Struct (JSON):");
    let json = serde_json::to_string_pretty(&user_struct).unwrap();
    println!("{}", json);
}

/// Build a User struct with various field types
fn build_user_struct() -> StructDef {
    StructDef::new("User")
        .with_doc("Represents a user account in the system")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_field(
            FieldDef::new("id", RustType::String)
                .with_doc("Unique identifier for the user")
                .with_serde_rename("userId")
        )
        .with_field(
            FieldDef::new("email", RustType::String)
                .with_doc("User's email address")
                .required()
        )
        .with_field(
            FieldDef::new("name", RustType::String)
                .with_doc("User's display name")
                .optional()
        )
        .with_field(
            FieldDef::new("age", RustType::I32)
                .optional()
        )
        .with_field(
            FieldDef::new("roles", RustType::Vec(Box::new(RustType::String)))
                .with_doc("User roles")
        )
        .with_field(
            FieldDef::new("metadata", RustType::string_map(RustType::JsonValue))
                .with_doc("Additional metadata")
                .optional()
        )
}

/// Build a Status enum with different variant types
fn build_status_enum() -> EnumDef {
    EnumDef::new("UserStatus")
        .with_doc("Represents the current status of a user")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_variant(
            VariantDef::unit("Active")
                .with_doc("User account is active")
                .as_default()
        )
        .with_variant(
            VariantDef::unit("Inactive")
                .with_doc("User account is inactive")
        )
        .with_variant(
            VariantDef::tuple("Suspended", RustType::String)
                .with_doc("User account is suspended with a reason")
        )
        .with_variant(
            VariantDef::struct_variant(
                "Pending",
                vec![
                    FieldDef::new("verification_code", RustType::String),
                    FieldDef::new("expires_at", RustType::String),
                ]
            )
            .with_doc("User account is pending verification")
        )
}

/// Build a BlockchainKind enum (similar to what's in the current generated.rs)
fn build_blockchain_enum() -> EnumDef {
    EnumDef::new("BlockchainKind")
        .with_doc("Supported blockchain types")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_variant(VariantDef::unit("Algorand").as_default())
        .with_variant(VariantDef::unit("Aptos"))
        .with_variant(VariantDef::unit("Bitcoin"))
        .with_variant(VariantDef::unit("Evm").with_serde_rename("EVM"))
        .with_variant(VariantDef::unit("Solana"))
}

/// Build a complete code generation output with multiple types and modules
fn build_complete_output() -> CodeGenOutput {
    CodeGenOutput::new()
        .with_import("use serde::{Deserialize, Serialize};".to_string())
        .with_import("use std::collections::HashMap;".to_string())
        .with_attribute("#![allow(clippy::all)]".to_string())
        // Top-level types
        .with_type(TypeDef::Struct(build_user_struct()))
        .with_type(TypeDef::Enum(build_blockchain_enum()))
        // Module with related types
        .with_module(
            ModuleDef::new("auth")
                .with_doc("Authentication-related types")
                .with_type(TypeDef::Enum(build_status_enum()))
                .with_type(TypeDef::Struct(
                    StructDef::new("LoginRequest")
                        .with_field(FieldDef::new("email", RustType::String))
                        .with_field(FieldDef::new("password", RustType::String))
                ))
        )
}
