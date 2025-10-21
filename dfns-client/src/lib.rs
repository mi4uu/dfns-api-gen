// Generated DFNS API types
pub mod generated;

// Generated API endpoints (TODO: Update endpoint generator to use nested modules)
// pub mod generated_api;

// DFNS HTTP Client
pub mod client;

// Re-export commonly used types
pub use client::{DfnsClient, config::DfnsConfig, error::{DfnsClientError, DfnsClientResult}};
