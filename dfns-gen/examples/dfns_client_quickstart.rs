// DFNS Client Quick Start Example
//
// This shows the most common use cases in a simple, copy-paste friendly format

use dfns_gen::client::{DfnsClient, config::DfnsConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct WalletListResponse {
    items: Vec<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup client
    let config = DfnsConfig::new(
        "https://api.dfns.ninja",
        "your-service-account-token",
        "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
    );
    let client = DfnsClient::new(config)?;

    // 2. Service account request (simple)
    let wallets: WalletListResponse = client
        .service_account()
        .get("/wallets")
        .await?;
    println!("✓ Found {} wallets", wallets.items.len());

    // 3. User impersonation + simple GET
    let user_client = client
        .as_user("user@example.com")
        .authenticate()
        .await?;
    println!("✓ Authenticated as user");

    let wallet: serde_json::Value = user_client
        .get("/wallets/wa-123456")
        .await?;
    println!("✓ Got wallet: {:?}", wallet);

    // 4. User operation with challenge (3-step flow)
    let transfer_data = serde_json::json!({
        "to": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
        "amount": "1000000000000000000"
    });

    let flow = user_client
        .put_with_challenge::<_, serde_json::Value>("/wallets/wa-123456/transfers", &transfer_data)
        .await?;

    // Option A: Complete with user signature (client-side)
    // let signed = SignedChallenge { ... };
    // let result = flow.complete_with_user_signature(signed).await?;

    // Option B: Complete with service account (server-side automation)
    // let result = flow.complete_with_service_account().await?;

    println!("✓ Challenge flow initiated");

    Ok(())
}
