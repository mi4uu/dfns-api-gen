/// DFNS Client Configuration
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfnsConfig {
    /// Base API URL (e.g., "https://api.dfns.ninja")
    pub api_url: String,
    /// Service account bearer token
    pub service_account_token: String,
    /// Private key for signing (PEM format)
    pub private_key: String,
    /// Public key ID (optional, extracted from allowed credentials)
    pub public_key_id: Option<String>,
}

impl DfnsConfig {
    /// Create a new DFNS configuration
    pub fn new(api_url: impl Into<String>, service_account_token: impl Into<String>, private_key: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into(),
            service_account_token: service_account_token.into(),
            private_key: private_key.into(),
            public_key_id: None,
        }
    }

    /// Set the public key ID
    pub fn with_public_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.public_key_id = Some(key_id.into());
        self
    }

    /// Create configuration from environment variables
    ///
    /// Expected variables:
    /// - DFNS_API_URL
    /// - DFNS_SERVICE_ACCOUNT_TOKEN
    /// - DFNS_PRIVATE_KEY
    /// - DFNS_PUBLIC_KEY_ID (optional)
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            api_url: std::env::var("DFNS_API_URL")?,
            service_account_token: std::env::var("DFNS_SERVICE_ACCOUNT_TOKEN")?,
            private_key: std::env::var("DFNS_PRIVATE_KEY")?,
            public_key_id: std::env::var("DFNS_PUBLIC_KEY_ID").ok(),
        })
    }
}
