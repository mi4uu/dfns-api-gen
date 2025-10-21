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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr: SocketAddr = "0.0.0.0:5555".parse().unwrap();

    // Print to stdout immediately
    println!("🚀 DFNS API Mock Server starting...");
    println!("📍 Address: {}", addr);
    println!("📚 Swagger UI: http://{}/swagger-ui", addr);
    println!();

    // Build the API router with generated endpoints
    let api_router = dfns_client::generated_api::ApiDoc::router();

    let app = Router::new()
        .merge(api_router)
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", dfns_client::generated_api::ApiDoc::openapi()));

    println!("🔧 Binding to {}...", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("✅ Server is running!");
    println!("   → Swagger UI: http://localhost:5555/swagger-ui");
    println!("   → OpenAPI JSON: http://localhost:5555/api-docs/openapi.json");
    println!("   → All API endpoints are now available!");
    println!();

    tracing::info!("Server started successfully on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
