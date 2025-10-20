# Dfns OpenAPI to Rust Generator

A Rust code generator that converts OpenAPI 3.1.x schemas into idiomatic Rust structs and enums with full serde support.

## Features

- **Complete Schema Coverage**: Extracts types from both `components/schemas` AND inline schemas in API paths (request/response bodies)
- **Automatic Type Generation**: Converts OpenAPI schemas into Rust types
- **Full Serde Support**: Generated types include `Serialize` and `Deserialize` derives
- **Proper Naming Conventions**: 
  - Snake_case for struct fields
  - PascalCase for types and enum variants
  - Automatic `#[serde(rename)]` attributes when needed
- **Optional Fields**: Generates `Option<T>` for non-required fields with `skip_serializing_if`
- **Enum Support**: Handles string enums and `oneOf` patterns
- **Complex Types**: Supports nested objects, arrays, references, and `allOf` composition
- **Type Mapping**: Maps OpenAPI types to appropriate Rust types (String, i32, i64, f32, f64, bool, Vec<T>)
- **Deterministic Output**: Same OpenAPI spec always generates identical Rust code

## Usage

### Generating Types

Run the generator to fetch the Dfns OpenAPI spec and generate Rust types:

```bash
cargo run
```

This will:
1. Fetch the OpenAPI spec from `https://docs.dfns.co/openapi.yaml`
2. Save it as `openapi.json`
3. Generate Rust code in `src/generated.rs` (~2,600 lines, 181 types from 117 API endpoints)

### Using Generated Types

The generated types are fully compatible with serde for JSON serialization/deserialization:

```rust
use dfns_gen::generated::*;

fn main() {
    // Create and use enums
    let blockchain = BlockchainKind::Evm;
    let json = serde_json::to_string(&blockchain).unwrap();
    println!("Serialized: {}", json); // "Evm"
    
    // Create and use structs
    let validator = CantonValidator {
        date_created: "2025-10-20T00:00:00Z".to_string(),
        id: "cv-12345".to_string(),
        kind: "Validator".to_string(),
        name: Some("My Validator".to_string()),
        network: "CantonTestnet".to_string(),
        org_id: "or-12345-67890-abcdefghijklmnop".to_string(),
        party_hint: "party::validator::1".to_string(),
    };
    
    let json = serde_json::to_string_pretty(&validator).unwrap();
    println!("Serialized:\n{}", json);
}
```

See `examples/demo.rs` for a complete example:

```bash
cargo run --example demo
```

## Architecture

The generator is built with a modular architecture:

- **`codegen/mod.rs`**: Main generator orchestration
- **`codegen/schema_generator.rs`**: Converts OpenAPI schemas to Rust structs/enums
- **`codegen/type_mapper.rs`**: Maps OpenAPI types to Rust types
- **`codegen/utils.rs`**: Naming convention helpers and utilities

## Generated Code Features

### Component Schemas (Reusable Types)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockchainKind {
    Algorand,
    Aptos,
    Bitcoin,
    // ... more variants
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CantonValidator {
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    
    pub id: String,
    
    /// Optional fields with skip_serializing_if
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// Organization id.
    #[serde(rename = "orgId")]
    pub org_id: String,
}
```

### API Path Types (Request/Response Bodies)

```rust
// Request body for POST /auth/action/init
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthActionInitPostRequest {
    #[serde(rename = "userActionHttpMethod")]
    pub user_action_http_method: String,
    
    #[serde(rename = "userActionHttpPath")]
    pub user_action_http_path: String,
    
    #[serde(rename = "userActionPayload")]
    pub user_action_payload: String,
}

// Response body for GET /agreements/latest-unaccepted (200)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgreementsLatestUnacceptedGetResponse200 {
    #[serde(rename = "latestAgreement")]
    pub latest_agreement: serde_json::Value,
}
```

## Comparison to oas3-gen

This project was inspired by [oas3-gen](https://github.com/eklipse2k8/oas3-gen) but with key improvements:

- **Working Implementation**: Fully functional code generation without runtime crashes
- **Library + Binary**: Can be used both as a library and standalone tool
- **Better Type Handling**: Proper support for OpenAPI 3.1.x `TypeSet` and complex schemas
- **Modular Design**: Clean separation of concerns for maintainability

## Dependencies

- `oas3`: OpenAPI 3.1.x parser
- `serde`: Serialization framework
- `serde_json`: JSON support
- `reqwest`: HTTP client for fetching OpenAPI specs
- `tokio`: Async runtime

## License

MIT
