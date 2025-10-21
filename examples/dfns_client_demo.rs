// Complete DFNS Client Usage Examples
//
// This demonstrates the full capabilities of the DFNS client:
// 1. Service account authentication
// 2. User impersonation
// 3. Simple requests
// 4. 3-step challenge/signing flow
// 5. Best practices and patterns

use dfns_gen::client::{DfnsClient, config::DfnsConfig};
use serde::{Deserialize, Serialize};

// Example response types (would normally be generated from OpenAPI)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Wallet {
    id: String,
    name: String,
    network: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WalletListResponse {
    items: Vec<Wallet>,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferRequest {
    to: String,
    amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferResponse {
    id: String,
    status: String,
    tx_hash: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DFNS Client Examples ===\n");

    // Initialize configuration
    let config = DfnsConfig::new(
        "https://api.dfns.ninja",
        "your-service-account-token",
        "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
    );

    let client = DfnsClient::new(config)?;

    // Example 1: Service Account Requests
    println!("📋 Example 1: Service Account Request (List Wallets)");
    example_service_account_request(&client).await?;

    // Example 2: User Impersonation
    println!("\n👤 Example 2: User Impersonation");
    example_user_authentication(&client).await?;

    // Example 3: Simple User Request (No Challenge)
    println!("\n📊 Example 3: Simple User Request (Get Wallet)");
    example_simple_user_request(&client).await?;

    // Example 4: 3-Step Challenge Flow (User Signing)
    println!("\n🔐 Example 4: 3-Step Challenge Flow (Transfer)");
    example_challenge_flow_user_signing(&client).await?;

    // Example 5: 3-Step Challenge Flow (Service Account Signing)
    println!("\n🔑 Example 5: 3-Step Challenge Flow (Service Account Signing)");
    example_challenge_flow_service_signing(&client).await?;

    println!("\n✅ All examples completed!");

    Ok(())
}

/// Example 1: Simple service account authenticated request
async fn example_service_account_request(client: &DfnsClient) -> Result<(), Box<dyn std::error::Error>> {
    // Service account requests are the simplest - just authenticate and call
    let wallets: WalletListResponse = client
        .service_account()
        .get("/wallets")
        .await?;

    println!("   Found {} wallets", wallets.items.len());
    for wallet in wallets.items.iter().take(3) {
        println!("   - {} ({})", wallet.name, wallet.id);
    }

    Ok(())
}

/// Example 2: Authenticate as a user (impersonation)
async fn example_user_authentication(client: &DfnsClient) -> Result<(), Box<dyn std::error::Error>> {
    // Impersonate a user - this does the 3-step flow internally
    let user_client = client
        .as_user("user@example.com")
        .authenticate()
        .await?;

    println!("   ✓ Successfully authenticated as user@example.com");

    // Now you can use user_client for user-scoped requests
    Ok(())
}

/// Example 3: Simple user request (no challenge required)
async fn example_simple_user_request(client: &DfnsClient) -> Result<(), Box<dyn std::error::Error>> {
    let user_client = client
        .as_user("user@example.com")
        .authenticate()
        .await?;

    // Simple GET request doesn't require challenge/signing
    let wallet: Wallet = user_client
        .get("/wallets/wa-123456")
        .await?;

    println!("   Wallet: {} (network: {})", wallet.name, wallet.network);

    Ok(())
}

/// Example 4: 3-Step challenge flow with user signing (client-side)
async fn example_challenge_flow_user_signing(client: &DfnsClient) -> Result<(), Box<dyn std::error::Error>> {
    let user_client = client
        .as_user("user@example.com")
        .authenticate()
        .await?;

    let transfer_request = TransferRequest {
        to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
        amount: "1000000000000000000".to_string(), // 1 ETH in wei
        memo: Some("Payment for services".to_string()),
    };

    // Step 1: Initialize the challenge flow
    let flow = user_client
        .put_with_challenge::<_, TransferResponse>("/wallets/wa-123456/transfers", &transfer_request)
        .await?;

    println!("   📝 Challenge initialized");
    let challenge = flow.challenge();
    println!("   Challenge ID: {}", challenge.challenge_identifier);

    // Step 2: In a real application, you would:
    // - Send challenge.challenge to the client (browser/mobile app)
    // - Client signs with WebAuthn/Passkey
    // - Client sends back the signed challenge

    // For this example, we'll simulate the signed response
    use dfns_gen::client::challenge::SignedChallenge;
    let signed = SignedChallenge {
        credential_id: "cred-abc123".to_string(),
        client_data: "base64-encoded-client-data".to_string(),
        signature: "base64-encoded-signature".to_string(),
    };

    // Step 3: Complete the flow with the signed challenge
    println!("   ✍️  Completing with user signature...");
    // let result = flow.complete_with_user_signature(signed).await?;
    // println!("   ✅ Transfer completed! ID: {}", result.id);

    println!("   (Simulated - would complete with real signature)");

    Ok(())
}

/// Example 5: 3-Step challenge flow with service account signing (server-side)
async fn example_challenge_flow_service_signing(client: &DfnsClient) -> Result<(), Box<dyn std::error::Error>> {
    let user_client = client
        .as_user("user@example.com")
        .authenticate()
        .await?;

    let transfer_request = TransferRequest {
        to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
        amount: "1000000000000000000".to_string(),
        memo: Some("Automated transfer".to_string()),
    };

    // Initialize the challenge flow
    let flow = user_client
        .put_with_challenge::<_, TransferResponse>("/wallets/wa-123456/transfers", &transfer_request)
        .await?;

    println!("   📝 Challenge initialized");

    // Complete with service account signing (server-side)
    // This is useful when you want to automate operations without user interaction
    println!("   🔑 Signing with service account key...");
    // let result = flow.complete_with_service_account().await?;
    // println!("   ✅ Transfer completed! ID: {}", result.id);

    println!("   (Simulated - would complete with service account signature)");

    Ok(())
}

// ============================================================================
// REAL-WORLD USAGE PATTERNS
// ============================================================================

/// Pattern 1: Service-side wallet operations for users
#[allow(dead_code)]
async fn pattern_service_wallet_operations(
    client: &DfnsClient,
    user_id: &str,
) -> Result<Vec<Wallet>, Box<dyn std::error::Error>> {
    // As a service, you can list wallets for a user with service account auth
    let response: WalletListResponse = client
        .service_account()
        .get(&format!("/wallets?owner={}", user_id))
        .await?;

    Ok(response.items)
}

/// Pattern 2: User-initiated operations with client-side signing
#[allow(dead_code)]
async fn pattern_user_initiated_transfer(
    client: &DfnsClient,
    username: &str,
    wallet_id: &str,
    to_address: &str,
    amount: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Authenticate as user
    let user_client = client.as_user(username).authenticate().await?;

    // Create transfer request
    let request = TransferRequest {
        to: to_address.to_string(),
        amount: amount.to_string(),
        memo: None,
    };

    // Initialize challenge
    let flow = user_client
        .put_with_challenge::<_, TransferResponse>(&format!("/wallets/{}/transfers", wallet_id), &request)
        .await?;

    // Return challenge to client for signing
    let challenge_id = flow.challenge().challenge_identifier.clone();

    // In production, you would:
    // 1. Store the flow state
    // 2. Return challenge to client
    // 3. Wait for signed response
    // 4. Complete the flow

    Ok(challenge_id)
}

/// Pattern 3: Automated operations (service account signing)
#[allow(dead_code)]
async fn pattern_automated_operations(
    client: &DfnsClient,
    username: &str,
    operation_data: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // Impersonate user
    let user_client = client.as_user(username).authenticate().await?;

    // Initialize and complete in one flow (service signs)
    let flow = user_client
        .post_with_challenge::<_, serde_json::Value>("/some/endpoint", &operation_data)
        .await?;

    let _result = flow.complete_with_service_account().await?;

    Ok(())
}

/// Pattern 4: Mixed authentication - read with service, write with user
#[allow(dead_code)]
async fn pattern_mixed_auth(
    client: &DfnsClient,
    user_id: &str,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read operations: Use service account (faster, no user interaction)
    let wallets: WalletListResponse = client
        .service_account()
        .get(&format!("/wallets?owner={}", user_id))
        .await?;

    println!("User has {} wallets", wallets.items.len());

    // Write operations: Use user authentication (requires user consent)
    let user_client = client.as_user(username).authenticate().await?;

    // ... perform user-authorized operations

    Ok(())
}
