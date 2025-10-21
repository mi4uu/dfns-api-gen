// DFNS API Mock Server
//
// This binary provides a mock implementation of the DFNS API
// using the generated types and endpoints.
//
// Run with: cargo run -p dfns-client --bin dfns-api-server

use axum::Router;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the API router
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // TODO: Merge generated API routes when they're properly generated
    // .merge(dfns_client::generated_api::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("🚀 DFNS API Mock Server listening on {}", addr);
    tracing::info!("📚 Swagger UI available at http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "DFNS API",
        version = "1.0.0",
        description = "Generated DFNS API with types and endpoints"
    ),
    tags(
        (name = "wallets", description = "Wallet operations"),
        (name = "auth", description = "Authentication"),
        (name = "users", description = "User management")
    )
)]
struct ApiDoc;
