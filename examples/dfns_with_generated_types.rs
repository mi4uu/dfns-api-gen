// Example: Using DFNS Client with Generated Types
//
// This shows how to use the DFNS client with types generated from OpenAPI spec.
// Assumes you've run the generator to create types in src/generated.rs

// Uncomment when you have generated types:
// use dfns_gen::generated::wallets::{ListResponse, CreateRequest};
// use dfns_gen::generated::wallets::wallet_id::{GetResponse, UpdateRequest};
// use dfns_gen::generated::wallets::wallet_id::transfers::CreateRequest as TransferRequest;

use dfns_gen::client::{DfnsClient, config::DfnsConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DFNS Client with Generated Types Example ===\n");

    // Setup
    let config = DfnsConfig::new(
        "https://api.dfns.ninja",
        "your-service-account-token",
        "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
    );
    let client = DfnsClient::new(config)?;

    // ========================================================================
    // Pattern 1: Service Account Requests with Generated Types
    // ========================================================================
    println!("📋 Pattern 1: List Wallets (Service Account)\n");

    // With generated types, you get full type safety:
    // let wallets: ListResponse = client
    //     .service_account()
    //     .get("/wallets")
    //     .await?;
    //
    // for wallet in wallets.items {
    //     println!("  - {} ({})", wallet.name, wallet.id);
    // }

    // ========================================================================
    // Pattern 2: Creating Resources with Request Types
    // ========================================================================
    println!("\n🔨 Pattern 2: Create Wallet (Service Account)\n");

    // Generated request types ensure you provide all required fields:
    // let create_req = CreateRequest {
    //     name: "My New Wallet".to_string(),
    //     network: "EthereumSepolia".to_string(),
    //     // TypeScript/OpenAPI → Rust conversion handles:
    //     // - Required fields (compile error if missing)
    //     // - Optional fields (Option<T>)
    //     // - Field renaming (camelCase ↔ snake_case)
    // };
    //
    // let wallet: GetResponse = client
    //     .service_account()
    //     .post("/wallets", &create_req)
    //     .await?;
    //
    // println!("  Created wallet: {}", wallet.id);

    // ========================================================================
    // Pattern 3: User Operations with Nested Module Types
    // ========================================================================
    println!("\n👤 Pattern 3: User Operations with Nested Types\n");

    // Authenticate as user
    let user_client = client
        .as_user("user@example.com")
        .authenticate()
        .await?;

    // Get wallet details (nested module: wallets::wallet_id::GetResponse)
    // let wallet: GetResponse = user_client
    //     .get("/wallets/wa-123456")
    //     .await?;
    //
    // println!("  Wallet: {} on {}", wallet.name, wallet.network);
    // println!("  Status: {}", wallet.status);

    // ========================================================================
    // Pattern 4: 3-Step Flow with Transfer Types
    // ========================================================================
    println!("\n🔐 Pattern 4: Transfer with Challenge Flow\n");

    // Generated transfer request type
    // let transfer = TransferRequest {
    //     to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
    //     amount: "1000000000000000000".to_string(),
    //     asset_symbol: Some("ETH".to_string()),
    //     // All fields are type-checked!
    // };

    // Initialize challenge flow with typed response
    // let flow = user_client
    //     .put_with_challenge::<_, TransferResponse>(
    //         "/wallets/wa-123456/transfers",
    //         &transfer
    //     )
    //     .await?;
    //
    // // Challenge is type-safe
    // let challenge = flow.challenge();
    // println!("  Challenge ID: {}", challenge.challenge_identifier);
    //
    // // Complete and get typed response
    // let result = flow.complete_with_service_account().await?;
    // println!("  Transfer ID: {}", result.id);
    // println!("  Status: {}", result.status);

    // ========================================================================
    // Pattern 5: Handling Response Enums
    // ========================================================================
    println!("\n🔀 Pattern 5: Handling Enum Response Types\n");

    // Generated enums for response variants:
    // match wallet.status {
    //     WalletStatus::Active => println!("  Wallet is active"),
    //     WalletStatus::Pending => println!("  Wallet is pending"),
    //     WalletStatus::Failed => println!("  Wallet failed"),
    // }

    // ========================================================================
    // Benefits of Generated Types
    // ========================================================================
    println!("\n✅ Benefits of Generated Types:\n");
    println!("  1. Compile-time safety - can't miss required fields");
    println!("  2. IDE autocomplete - know what fields exist");
    println!("  3. Refactoring safety - compiler catches API changes");
    println!("  4. Documentation - types document the API");
    println!("  5. Validation - Serde handles serialization/deserialization");
    println!("  6. Clean naming - nested modules organize types logically");

    Ok(())
}

// ============================================================================
// Example: Building a Type-Safe API Wrapper
// ============================================================================

/// Type-safe wallet operations wrapper
#[allow(dead_code)]
pub struct WalletClient {
    client: DfnsClient,
}

#[allow(dead_code)]
impl WalletClient {
    pub fn new(config: DfnsConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let client = DfnsClient::new(config)?;
        Ok(Self { client })
    }

    // List all wallets (service account)
    // pub async fn list_wallets(&self) -> Result<ListResponse, Box<dyn std::error::Error>> {
    //     let response = self.client.service_account().get("/wallets").await?;
    //     Ok(response)
    // }

    // Get wallet by ID (service account)
    // pub async fn get_wallet(&self, wallet_id: &str) -> Result<GetResponse, Box<dyn std::error::Error>> {
    //     let response = self.client
    //         .service_account()
    //         .get(&format!("/wallets/{}", wallet_id))
    //         .await?;
    //     Ok(response)
    // }

    // Create wallet (service account)
    // pub async fn create_wallet(&self, request: CreateRequest) -> Result<GetResponse, Box<dyn std::error::Error>> {
    //     let response = self.client
    //         .service_account()
    //         .post("/wallets", &request)
    //         .await?;
    //     Ok(response)
    // }

    // Transfer funds (user operation with challenge)
    // pub async fn transfer(
    //     &self,
    //     username: &str,
    //     wallet_id: &str,
    //     request: TransferRequest,
    // ) -> Result<ChallengeFlow<TransferResponse>, Box<dyn std::error::Error>> {
    //     let user_client = self.client.as_user(username).authenticate().await?;
    //     let flow = user_client
    //         .put_with_challenge(&format!("/wallets/{}/transfers", wallet_id), &request)
    //         .await?;
    //     Ok(flow)
    // }
}

// ============================================================================
// Example: API Route Handler (Axum)
// ============================================================================

#[allow(dead_code)]
mod api_routes {
    use super::*;
    use axum::{Json, extract::{State, Path}};
    // use crate::generated::wallets::CreateRequest;
    // use crate::generated::wallets::wallet_id::GetResponse;

    #[derive(Clone)]
    pub struct AppState {
        dfns_client: DfnsClient,
    }

    // GET /wallets/:id
    // pub async fn get_wallet(
    //     State(state): State<AppState>,
    //     Path(wallet_id): Path<String>,
    // ) -> Result<Json<GetResponse>, StatusCode> {
    //     let wallet = state.dfns_client
    //         .service_account()
    //         .get(&format!("/wallets/{}", wallet_id))
    //         .await
    //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //
    //     Ok(Json(wallet))
    // }

    // POST /wallets
    // pub async fn create_wallet(
    //     State(state): State<AppState>,
    //     Json(request): Json<CreateRequest>,
    // ) -> Result<Json<GetResponse>, StatusCode> {
    //     let wallet = state.dfns_client
    //         .service_account()
    //         .post("/wallets", &request)
    //         .await
    //         .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //
    //     Ok(Json(wallet))
    // }
}

// ============================================================================
// Key Takeaways
// ============================================================================

/*

1. GENERATED TYPES ARE YOUR FRIEND
   - Compile-time safety
   - IDE support
   - Self-documenting

2. NESTED MODULES = CLEAN ORGANIZATION
   - wallets::ListResponse
   - wallets::wallet_id::GetResponse
   - wallets::wallet_id::transfers::CreateRequest

3. BUILDER PATTERN = GREAT DX
   - client.service_account().get()
   - client.as_user().authenticate().get()
   - flow.complete_with_user_signature()

4. TYPE PARAMETERS PRESERVE TYPE INFO
   - put_with_challenge::<_, TransferResponse>()
   - ChallengeFlow<T> knows the response type
   - No casting or generic JSON

5. PATTERN: SERVICE FOR READS, USER FOR WRITES
   - Service account: Fast, no user interaction
   - User operations: Require consent/signature
   - Mix them for optimal UX

*/
