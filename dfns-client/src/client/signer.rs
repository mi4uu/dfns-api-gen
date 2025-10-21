/// Request signing with Ed25519 keys
use super::challenge::{UserAction, UserActionChallenge};
use super::error::{DfnsClientError, DfnsClientResult};
use super::DfnsClient;
use base64::prelude::*;
use josekit::jws::EdDSA;
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// Sign a user action challenge with the service account private key
pub async fn sign_challenge(
    client: &DfnsClient,
    challenge: &UserActionChallenge,
    extra_headers: &HashMap<String, String>,
) -> DfnsClientResult<UserAction> {
    // Get the allowed key credential
    let allowed_key = challenge
        .allow_credentials
        .key
        .first()
        .ok_or(DfnsClientError::NoAllowedCredentials)?;

    // Create client data JSON
    let client_data = json!({
        "challenge": &challenge.challenge,
        "type": "key.get"
    })
    .to_string();

    // Sign with Ed25519 private key
    let signer = EdDSA.signer_from_pem(&client.config.private_key)?;
    let signature = signer.sign(client_data.as_bytes())?;

    // Base64URL encode
    let client_data_b64 = BASE64_URL_SAFE.encode(&client_data);
    let signature_b64 = BASE64_URL_SAFE.encode(&signature);

    // Build first factor
    let first_factor = json!({
        "kind": "Key",
        "credentialAssertion": {
            "credId": allowed_key.id,
            "clientData": client_data_b64,
            "signature": signature_b64,
        },
    });

    // Complete the action
    let action_body = json!({
        "challengeIdentifier": challenge.challenge_identifier,
        "firstFactor": first_factor
    });

    let mut headers = extra_headers.clone();
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", client.config.service_account_token),
    );

    let response = client
        .send_request(Method::POST, "/auth/action", Some(action_body), &headers)
        .await?;

    let user_action: UserAction = response.json().await?;
    Ok(user_action)
}

/// Sign a challenge with user credentials (WebAuthn)
/// This is typically done client-side in browser, but we provide the structure
pub fn prepare_user_signature(
    challenge: &UserActionChallenge,
    credential_id: &str,
    client_data: &str,
    signature: &str,
) -> serde_json::Value {
    json!({
        "challengeIdentifier": challenge.challenge_identifier,
        "firstFactor": {
            "kind": "Key",
            "credentialAssertion": {
                "credId": credential_id,
                "clientData": client_data,
                "signature": signature,
            }
        }
    })
}
