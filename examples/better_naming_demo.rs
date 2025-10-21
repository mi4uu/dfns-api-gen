// Demonstration of improved naming and nested module structure
//
// Before: ServiceAccountsServiceAccountIdGETResponseUserInfoKind
// After:  service_accounts::service_account_id::GetResponse

use dfns_gen::codegen::ir_to_ast;
use dfns_gen::ir::*;

fn main() {
    println!("=== Better Naming & Module Structure Demo ===\n");

    // Build nested module structure for service accounts API
    let output = build_service_accounts_api();

    // Generate code
    let code = ir_to_ast::generate_code_output(&output);

    println!("Generated Code with Nested Modules:");
    println!("{}", "=".repeat(80));
    println!("{}", code);
    println!("{}", "=".repeat(80));

    println!("\n✅ Much better organization!");
    println!("\nUsage examples:");
    println!("  use api::service_accounts::{{CreateRequest, ListResponse}};");
    println!("  use api::service_accounts::service_account_id::GetResponse;");
    println!("  use api::service_accounts::service_account_id::actions::CreateRequest;");
}

fn build_service_accounts_api() -> CodeGenOutput {
    // Create root module for the API
    let mut service_accounts = ModuleDef::new("service_accounts")
        .with_doc("Service accounts management API");

    // Add top-level types (for /service_accounts endpoints)
    service_accounts.add_type(TypeDef::Struct(build_list_response()));
    service_accounts.add_type(TypeDef::Struct(build_create_request()));
    service_accounts.add_type(TypeDef::Struct(build_create_response()));

    // Add nested module for /service_accounts/{service_account_id}
    let mut service_account_id = ModuleDef::new("service_account_id")
        .with_doc("Operations on a specific service account");

    service_account_id.add_type(TypeDef::Struct(build_get_response()));
    service_account_id.add_type(TypeDef::Struct(build_update_request()));
    service_account_id.add_type(TypeDef::Struct(build_update_response()));
    service_account_id.add_type(TypeDef::Enum(build_user_info_kind()));

    // Add deeper nested module for /service_accounts/{service_account_id}/actions
    let mut actions = ModuleDef::new("actions")
        .with_doc("Service account actions");

    actions.add_type(TypeDef::Struct(build_action_create_request()));
    actions.add_type(TypeDef::Struct(build_action_create_response()));

    // Nest actions inside service_account_id
    service_account_id = service_account_id.with_submodule(actions);

    // Nest service_account_id inside service_accounts
    service_accounts = service_accounts.with_submodule(service_account_id);

    CodeGenOutput::new()
        .with_import("use serde::{Deserialize, Serialize};".to_string())
        .with_module(service_accounts)
}

// Top-level: GET /service_accounts (list)
fn build_list_response() -> StructDef {
    StructDef::new("ListResponse")
        .with_doc("Response for listing service accounts")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("items", RustType::Vec(Box::new(RustType::named("ServiceAccount"))))
                .required()
        )
        .with_field(
            FieldDef::new("next_page_token", RustType::String)
                .with_serde_rename("nextPageToken")
                .optional()
        )
}

// Top-level: POST /service_accounts
fn build_create_request() -> StructDef {
    StructDef::new("CreateRequest")
        .with_doc("Request to create a new service account")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("name", RustType::String)
                .with_doc("Service account name")
                .required()
        )
        .with_field(
            FieldDef::new("permissions", RustType::Vec(Box::new(RustType::String)))
                .optional()
        )
}

fn build_create_response() -> StructDef {
    StructDef::new("CreateResponse")
        .with_doc("Response after creating a service account")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("id", RustType::String)
                .with_doc("Created service account ID")
                .required()
        )
        .with_field(
            FieldDef::new("name", RustType::String)
                .required()
        )
}

// Nested: GET /service_accounts/{service_account_id}
fn build_get_response() -> StructDef {
    StructDef::new("GetResponse")
        .with_doc("Response for getting a specific service account")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("id", RustType::String)
                .required()
        )
        .with_field(
            FieldDef::new("name", RustType::String)
                .required()
        )
        .with_field(
            FieldDef::new("user_info_kind", RustType::named("UserInfoKind"))
                .with_serde_rename("userInfoKind")
                .required()
        )
        .with_field(
            FieldDef::new("created_at", RustType::String)
                .with_serde_rename("createdAt")
                .required()
        )
}

// Nested: PUT /service_accounts/{service_account_id}
fn build_update_request() -> StructDef {
    StructDef::new("UpdateRequest")
        .with_doc("Request to update a service account")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("name", RustType::String)
                .optional()
        )
        .with_field(
            FieldDef::new("permissions", RustType::Vec(Box::new(RustType::String)))
                .optional()
        )
}

fn build_update_response() -> StructDef {
    StructDef::new("UpdateResponse")
        .with_doc("Response after updating a service account")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("success", RustType::Bool)
                .required()
        )
}

// Enum used in service_account_id module
fn build_user_info_kind() -> EnumDef {
    EnumDef::new("UserInfoKind")
        .with_doc("Type of user information")
        .add_derive("utoipa::ToSchema".to_string())
        .with_variant(VariantDef::unit("EndUser").as_default())
        .with_variant(VariantDef::unit("ServiceAccount"))
        .with_variant(VariantDef::unit("Application"))
}

// Deeply nested: POST /service_accounts/{service_account_id}/actions
fn build_action_create_request() -> StructDef {
    StructDef::new("CreateRequest")
        .with_doc("Request to create an action for a service account")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("action_type", RustType::String)
                .with_serde_rename("actionType")
                .required()
        )
        .with_field(
            FieldDef::new("payload", RustType::JsonValue)
                .optional()
        )
}

fn build_action_create_response() -> StructDef {
    StructDef::new("CreateResponse")
        .with_doc("Response after creating an action")
        .add_derive("utoipa::ToSchema".to_string())
        .with_field(
            FieldDef::new("action_id", RustType::String)
                .with_serde_rename("actionId")
                .required()
        )
        .with_field(
            FieldDef::new("status", RustType::String)
                .required()
        )
}
