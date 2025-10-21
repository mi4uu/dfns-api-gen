# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`dfns_gen` is a Rust code generator that converts OpenAPI 3.1.x schemas into idiomatic Rust types with full serde support. It's designed specifically for the Dfns API but can be adapted for other OpenAPI specs. The project generates both type definitions (structs/enums) and API endpoint handlers with Axum and utoipa integration.

## Key Commands

### Building and Running

```bash
# Build the project
cargo build

# Run the main generator (fetches OpenAPI spec and generates types + API endpoints)
cargo run --bin convert

# Run the API server with Swagger UI (serves generated endpoints)
cargo run --bin api

# Run the selective typing utility
cargo run --bin selective

# Run the OpenAPI 3.1 to 3.0 downgrade tool
cargo run --bin downgrade

# Run the demo example
cargo run --example demo
```

### Testing

```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Check code without building
cargo check
```

### Linting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Architecture

### Code Generation Pipeline

The generator follows a three-stage pipeline:

1. **Fetch & Parse** (`src/main.rs`): Downloads OpenAPI spec from Dfns, saves as YAML/JSON
2. **Type Generation** (`src/codegen/`): Converts schemas to Rust structs/enums → `src/generated.rs`
3. **Endpoint Generation** (`src/codegen/endpoint_generator.rs`): Creates Axum handlers → `src/generated_api.rs`

### Module Structure

- **`codegen/mod.rs`**: Main generator orchestration. Extracts schemas from both `components/schemas` (reusable types) AND inline schemas in API paths (request/response bodies). Uses a modular approach where path-based types are organized into modules by endpoint.

- **`codegen/schema_generator.rs`**: Core type generation logic. Handles:
  - Enums (string enums and `oneOf` patterns with `#[serde(untagged)]`)
  - Structs (with proper field ordering via BTreeMap for deterministic output)
  - `allOf` composition (merges schemas into single struct)
  - Inline types (nested enums/structs within parent types)
  - Adds `utoipa::ToSchema` and `smart_default::SmartDefault` derives

- **`codegen/type_mapper.rs`**: Maps OpenAPI types to Rust types:
  - `string` → `String`
  - `integer` (format: int32) → `i32`
  - `integer` (format: int64) → `i64`
  - `number` (format: float) → `f32`
  - `number` (format: double) → `f64`
  - `boolean` → `bool`
  - `array` → `Vec<T>`
  - References → extracts type name from `$ref` path

- **`codegen/utils.rs`**: Naming convention helpers:
  - `to_snake_case()`: For struct field names
  - `to_pascal_case()`: For type names and enum variants
  - `sanitize_field_name()`: Handles Rust keywords (e.g., `type` → `r#type`)
  - `sanitize_variant_name()`: Cleans enum variant names

- **`codegen/endpoint_generator.rs`**: Generates Axum handlers with:
  - utoipa path annotations for OpenAPI docs
  - Type-safe request/response handling
  - Path parameter extraction
  - Mock implementations using examples from OpenAPI spec or `Default` trait

### Generated Code Organization

The generator creates two main files:

1. **`src/generated.rs`**: Type definitions
   - Top-level: Component schemas (from `#/components/schemas`)
   - Modules: Path-based schemas organized by endpoint (e.g., `pub mod wallets { ... }`)

2. **`src/generated_api.rs`**: API handlers
   - `ApiDoc` struct with OpenAPI metadata
   - Router setup for all endpoints
   - Handler functions with utoipa annotations

### API Server (`src/api.rs`)

The API server demonstrates how to use generated code:
- Serves all generated endpoints via Axum
- Swagger UI available at `/swagger`
- OpenAPI spec at `/openapi.json`
- Handlers return mock data (from examples or defaults)

## Important Implementation Details

### Deterministic Output

The generator ensures same input → same output:
- Uses `BTreeMap` for sorted field/property iteration
- Sorts component schemas alphabetically
- Consistent module naming from paths

### Type Generation Strategy

1. **Component Schemas**: Generated at top-level, globally accessible as `generated::TypeName`
2. **Path Schemas**: Generated in modules named after the first path segment (e.g., `/wallets/...` → `wallets` module)
3. **Inline Types**: Nested types are prefixed with parent name to avoid collisions (e.g., `CantonValidatorName` for inline enum in `CantonValidator.name`)

### Serde Attributes

Generated types include:
- `#[serde(rename = "...")]` when Rust field name differs from JSON key
- `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- `#[serde(untagged)]` for `oneOf` enums (allows deserializing from different JSON shapes)

### Path to Module/Type Naming

Paths are converted using this logic:
```
/wallets/{walletId}/export → module: "wallets", type prefix: "WalletIdExport"
/auth/action/init → module: "auth", type prefix: "ActionInit"
```

Then combined with HTTP method and request/response:
```
POST /auth/action/init → auth::ActionInitPostRequest, auth::ActionInitPostResponse
GET /wallets/{walletId}/export → wallets::WalletIdExportGetResponse
```

## Additional Binaries

### `downgrade` (src/downgrade.rs)
Converts OpenAPI 3.1 to 3.0 using `openapi31to30` crate. Useful for tools that don't support 3.1 yet.

### `selective` (src/selective_typing.rs)
Experimental: Allows selective type generation using `openapi_type_picker` with filter config. Currently configured to include all `userAction` types via `config.json`.

## Dependencies

Key crates and their purpose:
- `oas3`: OpenAPI 3.1.x parser
- `serde`, `serde_json`: Serialization
- `axum`: Web framework for API server
- `utoipa`, `utoipa-axum`, `utoipa-swagger-ui`: OpenAPI docs and Swagger UI
- `smart-default`: Default derive macro for enums
- `reqwest`, `tokio`: Async HTTP client and runtime

## Development Workflow

1. Modify code generation logic in `src/codegen/`
2. Run `cargo run --bin convert` to regenerate types
3. Check `src/generated.rs` and `src/generated_api.rs` for output
4. Test with `cargo run --bin api` and visit http://localhost:8080/swagger
5. Run `cargo test` to ensure nothing broke

## Current State

The repository has uncommitted changes:
- Modified: `Cargo.lock`, `Cargo.toml`
- Untracked: `config.json`, `src/selective_typing.rs`

The OpenAPI spec files (`openapi.json`, `openapi.yml`, `openapi30.json`, `openapi30.yml`) are generated artifacts and shouldn't be edited manually.
