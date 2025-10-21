// Generated DFNS API types (nested modules)
pub mod generated;

// Generated API endpoints (now uses nested modules!)
pub mod generated_api;

// DFNS HTTP Client
pub mod client;

// Re-export commonly used types
pub use client::{DfnsClient, config::DfnsConfig, error::{DfnsClientError, DfnsClientResult}};
