// Endpoint IR - tracks the relationship between OpenAPI operations and generated types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete intermediate representation including types and endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteIR {
    /// Map from schema name to its location in the generated module tree
    /// e.g., "Wallet" -> "generated::wallets::Wallet"
    pub schema_locations: HashMap<String, TypeLocation>,

    /// All endpoints with their metadata
    pub endpoints: Vec<EndpointMetadata>,

    /// All type definitions (from types.rs)
    pub type_modules: Vec<super::types::ModuleDef>,
}

/// Uniquely identifies an endpoint
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EndpointKey {
    pub method: String,  // "GET", "POST", etc.
    pub path: String,    // "/wallets/{walletId}"
}

/// Location of a generated type in the module tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeLocation {
    /// Full module path (e.g., ["wallets", "wallet_id"])
    pub module_path: Vec<String>,

    /// Type name within that module (e.g., "GetResponse", "CreateRequest")
    pub type_name: String,

    /// Full Rust path to use in code (e.g., "generated::wallets::wallet_id::GetResponse")
    pub full_path: String,
}

impl TypeLocation {
    pub fn new(module_path: Vec<String>, type_name: String) -> Self {
        let mut full_path = String::from("generated");
        for segment in &module_path {
            full_path.push_str("::");
            full_path.push_str(segment);
        }
        if !type_name.is_empty() {
            full_path.push_str("::");
            full_path.push_str(&type_name);
        }

        Self {
            module_path,
            type_name,
            full_path,
        }
    }
}

/// Metadata about an endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointMetadata {
    /// HTTP method
    pub method: String,

    /// API path
    pub path: String,

    /// Operation ID from OpenAPI (if present)
    pub operation_id: Option<String>,

    /// Request body type location (if any)
    pub request_type: Option<TypeLocation>,

    /// Response type location (if any)
    pub response_type: Option<TypeLocation>,

    /// HTTP status code for successful response
    pub response_status: String,

    /// Path parameters
    pub path_params: Vec<PathParameter>,

    /// Query parameters
    pub query_params: Vec<QueryParameter>,

    /// Summary/description
    pub summary: Option<String>,
    pub description: Option<String>,

    /// Tags for organization
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParameter {
    pub name: String,
    pub rust_type: String,  // "String", "i64", etc.
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParameter {
    pub name: String,
    pub rust_type: String,
    pub required: bool,
    pub description: Option<String>,
}

impl CompleteIR {
    pub fn new() -> Self {
        Self {
            schema_locations: HashMap::new(),
            endpoints: Vec::new(),
            type_modules: Vec::new(),
        }
    }

    /// Register a schema's location in the module tree
    pub fn register_schema_location(
        &mut self,
        schema_name: String,
        module_path: Vec<String>,
        type_name: String,
    ) {
        let location = TypeLocation::new(module_path, type_name);
        self.schema_locations.insert(schema_name, location);
    }

    /// Register an endpoint with its type mappings
    pub fn register_endpoint(&mut self, metadata: EndpointMetadata) {
        self.endpoints.push(metadata);
    }

    /// Get the type location for a request/response based on the endpoint
    pub fn get_endpoint_request_type(&self, method: &str, path: &str) -> Option<&TypeLocation> {
        self.endpoints.iter()
            .find(|e| e.method == method.to_uppercase() && e.path == path)?
            .request_type.as_ref()
    }

    pub fn get_endpoint_response_type(&self, method: &str, path: &str) -> Option<&TypeLocation> {
        self.endpoints.iter()
            .find(|e| e.method == method.to_uppercase() && e.path == path)?
            .response_type.as_ref()
    }

    /// Save IR to a JSON file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load IR from a JSON file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let ir = serde_json::from_str(&json)?;
        Ok(ir)
    }
}

impl Default for CompleteIR {
    fn default() -> Self {
        Self::new()
    }
}
