/// DFNS Client Error Types
use thiserror::Error;

pub type DfnsClientResult<T> = Result<T, DfnsClientError>;

#[derive(Debug, Error)]
pub enum DfnsClientError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Middleware error: {0}")]
    MiddlewareError(#[from] reqwest_middleware::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error (status {status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("User not authenticated - call authenticate() first")]
    NotAuthenticated,

    #[error("Signing error: {0}")]
    SigningError(String),

    #[error("Challenge error: {0}")]
    ChallengeError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("No allowed credentials found in challenge")]
    NoAllowedCredentials,

    #[error("{0}")]
    Other(String),
}

impl From<josekit::JoseError> for DfnsClientError {
    fn from(err: josekit::JoseError) -> Self {
        DfnsClientError::SigningError(err.to_string())
    }
}
