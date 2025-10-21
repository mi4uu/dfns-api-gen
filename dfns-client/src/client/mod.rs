// DFNS Client - Complete client implementation using generated types
//
// Features:
// - Service account authentication
// - User impersonation
// - Request signing (service + user)
// - 3-step challenge/signing flow
// - Builder pattern for excellent DX
// - Type-safe with generated types

pub mod config;
pub mod error;
pub mod signer;
pub mod challenge;

use config::DfnsConfig;
use error::{DfnsClientError, DfnsClientResult};
use reqwest::{Client, Method, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;

/// Main DFNS API client
#[derive(Clone)]
pub struct DfnsClient {
    /// HTTP client with middleware
    client: ClientWithMiddleware,
    /// Configuration (URL, tokens, keys)
    config: DfnsConfig,
    /// Extra headers to add to requests
    headers: HashMap<String, String>,
}

impl DfnsClient {
    /// Create a new DFNS client with configuration
    pub fn new(config: DfnsConfig) -> DfnsClientResult<Self> {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

        let client = Client::builder()
            .user_agent(format!("dfns-gen/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        let client = ClientBuilder::new(client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Ok(Self {
            client,
            config,
            headers: HashMap::new(),
        })
    }

    /// Create a request builder for service account authenticated requests
    pub fn service_account(&self) -> ServiceAccountRequestBuilder {
        ServiceAccountRequestBuilder {
            client: self.clone(),
            headers: HashMap::new(),
        }
    }

    /// Create a request builder for user-authenticated requests
    pub fn as_user(&self, username: &str) -> UserRequestBuilder {
        UserRequestBuilder {
            client: self.clone(),
            username: username.to_string(),
            user_token: None,
            headers: HashMap::new(),
        }
    }

    /// Low-level method to send a request
    pub(crate) async fn send_request(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
        headers: &HashMap<String, String>,
    ) -> DfnsClientResult<Response> {
        let url = format!("{}/{}", self.config.api_url.trim_end_matches('/'), path.trim_start_matches('/'));

        // Build request with reqwest_middleware
        let mut request = self.client.request(method, &url);

        // Add headers
        for (key, value) in headers {
            request = request.header(key.as_str(), value.as_str());
        }

        // Add body if present
        if let Some(body) = body {
            request = request.body(body.to_string())
                .header("Content-Type", "application/json");
        }

        let response = request.send().await?;

        // Check for errors
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(DfnsClientError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        Ok(response)
    }

    /// Clean all extra headers
    pub fn clean_headers(mut self) -> Self {
        self.headers.clear();
        self
    }

    /// Add a header to the client
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

/// Builder for service account authenticated requests
pub struct ServiceAccountRequestBuilder {
    client: DfnsClient,
    headers: HashMap<String, String>,
}

impl ServiceAccountRequestBuilder {
    /// Add an extra header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Execute a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> DfnsClientResult<T> {
        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.client.config.service_account_token));

        let response = self.client.send_request(Method::GET, path, None, &headers).await?;
        let data = response.json().await?;
        Ok(data)
    }

    /// Execute a POST request
    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> DfnsClientResult<T> {
        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.client.config.service_account_token));

        let body_json = serde_json::to_value(body)?;
        let response = self.client.send_request(Method::POST, path, Some(body_json), &headers).await?;
        let data = response.json().await?;
        Ok(data)
    }

    /// Execute a PUT request
    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> DfnsClientResult<T> {
        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.client.config.service_account_token));

        let body_json = serde_json::to_value(body)?;
        let response = self.client.send_request(Method::PUT, path, Some(body_json), &headers).await?;
        let data = response.json().await?;
        Ok(data)
    }

    /// Execute a DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> DfnsClientResult<T> {
        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.client.config.service_account_token));

        let response = self.client.send_request(Method::DELETE, path, None, &headers).await?;
        let data = response.json().await?;
        Ok(data)
    }

    /// Initialize a user action challenge (step 1 of 3-step flow)
    pub async fn init_user_action<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        payload: &B,
    ) -> DfnsClientResult<challenge::UserActionChallenge> {
        let payload_json = serde_json::to_value(payload)?;

        let init_body = serde_json::json!({
            "userActionPayload": payload_json.to_string(),
            "userActionHttpMethod": method.to_string().to_uppercase(),
            "userActionHttpPath": path,
        });

        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.client.config.service_account_token));

        let response = self.client.send_request(Method::POST, "/auth/action/init", Some(init_body), &headers).await?;
        let challenge = response.json().await?;
        Ok(challenge)
    }

    /// Sign a user action challenge with service account key (step 2)
    pub async fn sign_challenge(
        &self,
        challenge: &challenge::UserActionChallenge,
    ) -> DfnsClientResult<challenge::UserAction> {
        signer::sign_challenge(&self.client, challenge, &self.headers).await
    }
}

/// Builder for user-authenticated requests with 3-step flow support
pub struct UserRequestBuilder {
    client: DfnsClient,
    username: String,
    user_token: Option<String>,
    headers: HashMap<String, String>,
}

impl UserRequestBuilder {
    /// Authenticate as user (impersonation via delegated login)
    pub async fn authenticate(mut self) -> DfnsClientResult<Self> {
        // Step 1: Init user action for delegated login
        let payload = serde_json::json!({ "username": self.username });

        let service_builder = self.client.service_account();
        let challenge = service_builder
            .init_user_action(Method::POST, "/auth/login/delegated", &payload)
            .await?;

        // Step 2: Sign the challenge
        let signed = service_builder.sign_challenge(&challenge).await?;

        // Step 3: Complete login with user action
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.client.config.service_account_token));
        headers.insert("X-DFNS-USERACTION".to_string(), signed.user_action.clone());

        let response = self.client
            .send_request(Method::POST, "/auth/login/delegated", Some(payload), &headers)
            .await?;

        #[derive(Deserialize)]
        struct TokenResponse {
            token: String,
        }

        let token_response: TokenResponse = response.json().await?;
        self.user_token = Some(token_response.token);

        Ok(self)
    }

    /// Add an extra header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Execute a simple GET request (no 3-step flow)
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> DfnsClientResult<T> {
        let user_token = self.user_token.as_ref().ok_or(DfnsClientError::NotAuthenticated)?;

        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", user_token));

        let response = self.client.send_request(Method::GET, path, None, &headers).await?;
        let data = response.json().await?;
        Ok(data)
    }

    /// Execute a 3-step POST request with user challenge/signing
    pub async fn post_with_challenge<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> DfnsClientResult<challenge::ChallengeFlow<T>> {
        let user_token = self.user_token.as_ref().ok_or(DfnsClientError::NotAuthenticated)?;

        // Step 1: Initialize challenge
        let body_json = serde_json::to_value(body)?;
        let init_body = serde_json::json!({
            "userActionPayload": body_json.to_string(),
            "userActionHttpMethod": "POST",
            "userActionHttpPath": path,
        });

        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", user_token));

        let response = self.client
            .send_request(Method::POST, "/auth/action/init", Some(init_body), &headers)
            .await?;

        let challenge: challenge::UserActionChallenge = response.json().await?;

        // Return a flow builder for the client to complete
        Ok(challenge::ChallengeFlow::new(
            self.client.clone(),
            challenge,
            path.to_string(),
            body_json,
            user_token.clone(),
        ))
    }

    /// Execute a 3-step PUT request with user challenge/signing
    pub async fn put_with_challenge<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> DfnsClientResult<challenge::ChallengeFlow<T>> {
        let user_token = self.user_token.as_ref().ok_or(DfnsClientError::NotAuthenticated)?;

        let body_json = serde_json::to_value(body)?;
        let init_body = serde_json::json!({
            "userActionPayload": body_json.to_string(),
            "userActionHttpMethod": "PUT",
            "userActionHttpPath": path,
        });

        let mut headers = self.headers.clone();
        headers.insert("Authorization".to_string(), format!("Bearer {}", user_token));

        let response = self.client
            .send_request(Method::POST, "/auth/action/init", Some(init_body), &headers)
            .await?;

        let challenge: challenge::UserActionChallenge = response.json().await?;

        Ok(challenge::ChallengeFlow::new(
            self.client.clone(),
            challenge,
            path.to_string(),
            body_json,
            user_token.clone(),
        ))
    }
}
