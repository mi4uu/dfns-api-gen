/// 3-Step Challenge/Signing Flow Support
use super::error::{DfnsClientError, DfnsClientResult};
use super::DfnsClient;
use reqwest::Method;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::marker::PhantomData;

/// User action challenge returned by /auth/action/init
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserActionChallenge {
    pub challenge_identifier: String,
    pub challenge: String,
    pub allow_credentials: AllowCredentials,
    #[serde(default)]
    pub external_authentication_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowCredentials {
    #[serde(default)]
    pub webauthn: Vec<WebAuthnCredential>,
    #[serde(default)]
    pub key: Vec<KeyCredential>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    pub id: String,
    #[serde(rename = "type")]
    pub credential_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCredential {
    pub id: String,
    #[serde(rename = "type")]
    pub credential_type: String,
}

/// Completed user action response from /auth/action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAction {
    pub user_action: String,
}

/// Signed challenge structure (for user-side signing)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedChallenge {
    pub credential_id: String,
    pub client_data: String,
    pub signature: String,
}

/// 3-Step challenge flow builder
///
/// This represents a flow that has been initialized and needs to be completed
/// by the client (either server-side for service account or client-side for user)
pub struct ChallengeFlow<T> {
    client: DfnsClient,
    challenge: UserActionChallenge,
    path: String,
    body: serde_json::Value,
    user_token: String,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> ChallengeFlow<T> {
    pub(crate) fn new(
        client: DfnsClient,
        challenge: UserActionChallenge,
        path: String,
        body: serde_json::Value,
        user_token: String,
    ) -> Self {
        Self {
            client,
            challenge,
            path,
            body,
            user_token,
            _phantom: PhantomData,
        }
    }

    /// Get the challenge to send to the user for signing
    pub fn challenge(&self) -> &UserActionChallenge {
        &self.challenge
    }

    /// Complete the flow with a signed challenge from the user
    ///
    /// This is typically called after the user has signed the challenge
    /// client-side (e.g., with WebAuthn in browser)
    pub async fn complete_with_user_signature(self, signed: SignedChallenge) -> DfnsClientResult<T> {
        // Step 2: Complete /auth/action with user signature
        let first_factor = serde_json::json!({
            "kind": "Key",
            "credentialAssertion": {
                "credId": signed.credential_id,
                "clientData": signed.client_data,
                "signature": signed.signature,
            }
        });

        let action_body = serde_json::json!({
            "challengeIdentifier": self.challenge.challenge_identifier,
            "firstFactor": first_factor
        });

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.user_token));

        let response = self.client
            .send_request(Method::POST, "/auth/action", Some(action_body), &headers)
            .await?;

        let user_action: UserAction = response.json().await?;

        // Step 3: Execute the actual request with X-DFNS-USERACTION header
        headers.insert("X-DFNS-USERACTION".to_string(), user_action.user_action);

        let method = if self.path.contains("PUT") {
            Method::PUT
        } else {
            Method::POST
        };

        let response = self.client
            .send_request(method, &self.path, Some(self.body), &headers)
            .await?;

        let result: T = response.json().await?;
        Ok(result)
    }

    /// Complete the flow with service account signing (server-side)
    ///
    /// This signs the challenge with the service account private key
    /// and executes the request. Useful for server-side operations.
    pub async fn complete_with_service_account(self) -> DfnsClientResult<T> {
        // Sign the challenge with service account key
        let extra_headers = HashMap::new();
        let user_action = super::signer::sign_challenge(&self.client, &self.challenge, &extra_headers).await?;

        // Execute the actual request with X-DFNS-USERACTION header
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.user_token));
        headers.insert("X-DFNS-USERACTION".to_string(), user_action.user_action);

        let method = if self.path.contains("PUT") {
            Method::PUT
        } else {
            Method::POST
        };

        let response = self.client
            .send_request(method, &self.path, Some(self.body), &headers)
            .await?;

        let result: T = response.json().await?;
        Ok(result)
    }

    /// Cancel the flow without completing it
    pub fn cancel(self) {
        // Flow is dropped, no action taken
    }
}

/// Builder for completing user actions manually
///
/// Use this when you have more complex signing requirements
pub struct UserActionBuilder {
    client: DfnsClient,
    challenge_identifier: String,
    user_token: String,
}

impl UserActionBuilder {
    pub fn new(client: DfnsClient, challenge_identifier: String, user_token: String) -> Self {
        Self {
            client,
            challenge_identifier,
            user_token,
        }
    }

    /// Complete the user action with a signed challenge
    pub async fn complete(self, signed: SignedChallenge) -> DfnsClientResult<UserAction> {
        let first_factor = serde_json::json!({
            "kind": "Key",
            "credentialAssertion": {
                "credId": signed.credential_id,
                "clientData": signed.client_data,
                "signature": signed.signature,
            }
        });

        let action_body = serde_json::json!({
            "challengeIdentifier": self.challenge_identifier,
            "firstFactor": first_factor
        });

        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", self.user_token));

        let response = self.client
            .send_request(Method::POST, "/auth/action", Some(action_body), &headers)
            .await?;

        let user_action: UserAction = response.json().await?;
        Ok(user_action)
    }
}
