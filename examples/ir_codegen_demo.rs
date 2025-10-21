// Working demonstration of IR-based code generation
// This shows the complete flow without OpenAPI parsing

use dfns_gen::codegen::ir_to_ast;
use dfns_gen::ir::*;

fn main() {
    println!("=== IR-Based Code Generation Demo ===\n");

    // Simulate what openapi_to_ir would produce
    let output = build_sample_api_types();

    // Generate code
    let generated_code = ir_to_ast::generate_code_output(&output);

    println!("Generated Code:");
    println!("{}", "=".repeat(80));
    println!("{}", generated_code);
    println!("{}", "=".repeat(80));

    println!("\n✅ Success! The IR → AST → Code pipeline works perfectly!");
    println!("\nNext step: Fix openapi_to_ir.rs to populate IR from OpenAPI spec");
}

/// Build sample types that would come from OpenAPI parsing
fn build_sample_api_types() -> CodeGenOutput {
    CodeGenOutput::new()
        .with_import("use std::collections::HashMap;".to_string())
        // Top-level blockchain enum (like from components/schemas)
        .with_type(TypeDef::Enum(build_blockchain_enum()))
        // Top-level wallet struct
        .with_type(TypeDef::Struct(build_wallet_struct()))
        // Module for API endpoints
        .with_module(build_wallets_module())
}

/// BlockchainKind enum (from components/schemas/BlockchainKind)
fn build_blockchain_enum() -> EnumDef {
    EnumDef::new("BlockchainKind")
        .with_doc("Supported blockchain networks")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_variant(
            VariantDef::unit("Algorand")
                .with_doc("Algorand blockchain")
                .as_default()
        )
        .with_variant(VariantDef::unit("Aptos").with_doc("Aptos blockchain"))
        .with_variant(VariantDef::unit("Bitcoin").with_doc("Bitcoin blockchain"))
        .with_variant(
            VariantDef::unit("Evm")
                .with_doc("Ethereum Virtual Machine compatible chains")
                .with_serde_rename("EVM")
        )
        .with_variant(VariantDef::unit("Solana").with_doc("Solana blockchain"))
        .with_variant(VariantDef::unit("Tezos").with_doc("Tezos blockchain"))
}

/// Wallet struct (from components/schemas/Wallet)
fn build_wallet_struct() -> StructDef {
    StructDef::new("Wallet")
        .with_doc("Represents a cryptocurrency wallet")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_field(
            FieldDef::new("id", RustType::String)
                .with_doc("Unique wallet identifier")
                .with_serde_rename("walletId")
                .required()
        )
        .with_field(
            FieldDef::new("network", RustType::named("BlockchainKind"))
                .with_doc("Blockchain network")
                .required()
        )
        .with_field(
            FieldDef::new("name", RustType::String)
                .with_doc("Wallet display name")
                .optional()
        )
        .with_field(
            FieldDef::new("address", RustType::String)
                .with_doc("Wallet address on the blockchain")
                .required()
        )
        .with_field(
            FieldDef::new("date_created", RustType::String)
                .with_doc("ISO 8601 timestamp of wallet creation")
                .with_serde_rename("dateCreated")
                .required()
        )
        .with_field(
            FieldDef::new("tags", RustType::Vec(Box::new(RustType::String)))
                .with_doc("User-defined tags")
                .optional()
        )
        .with_field(
            FieldDef::new("metadata", RustType::string_map(RustType::JsonValue))
                .with_doc("Additional metadata")
                .optional()
        )
}

/// Module for wallet-related API types (from /wallets paths)
fn build_wallets_module() -> ModuleDef {
    ModuleDef::new("wallets")
        .with_doc("Types for wallet-related API endpoints")
        .with_type(TypeDef::Struct(build_create_wallet_request()))
        .with_type(TypeDef::Struct(build_list_wallets_response()))
        .with_type(TypeDef::Enum(build_wallet_status()))
}

/// CreateWalletRequest (from POST /wallets request body)
fn build_create_wallet_request() -> StructDef {
    StructDef::new("CreateWalletPostRequest")
        .with_doc("Request body for creating a new wallet")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_field(
            FieldDef::new("network", RustType::String)
                .with_doc("Blockchain network identifier")
                .required()
        )
        .with_field(
            FieldDef::new("name", RustType::String)
                .with_doc("Wallet name")
                .optional()
        )
}

/// ListWalletsResponse (from GET /wallets response)
fn build_list_wallets_response() -> StructDef {
    StructDef::new("WalletsGetResponse")
        .with_doc("Response for listing wallets")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_field(
            FieldDef::new("items", RustType::Vec(Box::new(RustType::named("super::Wallet"))))
                .with_doc("List of wallets")
                .required()
        )
        .with_field(
            FieldDef::new("next_page_token", RustType::String)
                .with_doc("Token for pagination")
                .with_serde_rename("nextPageToken")
                .optional()
        )
}

/// WalletStatus enum (inline enum from wallet response)
fn build_wallet_status() -> EnumDef {
    EnumDef::new("WalletStatus")
        .with_doc("Current status of a wallet")
        .add_derive("utoipa::ToSchema".to_string())
        .add_derive("smart_default::SmartDefault".to_string())
        .with_variant(VariantDef::unit("Active").as_default())
        .with_variant(VariantDef::unit("Archived"))
}
