use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes, PathItemExt};
#[derive(OpenApi)]
#[openapi(info(title = "dfns API client", version = "1.0.0",))]
pub struct Api;
pub mod generated_api;
use generated_api;
impl Api {
    pub fn get_router() -> OpenApiRouter {
        let main_router = OpenApiRouter::with_openapi(generated_api::ApiDoc::openapi());
        //   main_router.routes(routes!(get_wallet))
        main_router
    }
}
#[tokio::main]
async fn main() {
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5555").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

fn app() -> Router {
    let (router, api) = Api::get_router().split_for_parts();
    Router::new()
        .route("/", get(handler))
        .merge(router)
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger").url("/openapi.json", api.clone()))
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[utoipa::path(get, path = "/wallets", responses((status = OK, body = dfns_gen::generated::wallets::WalletIdAssetsGetResponse)))]
async fn get_wallet(
    Json(params): Json<dfns_gen::generated::wallets::WalletIdDelegatePostRequest>,
) -> Json<dfns_gen::generated::wallets::WalletIdAssetsGetResponse> {
    Json(dfns_gen::generated::wallets::WalletIdAssetsGetResponse {
        wallet_id: String::new(),
        net_worth: None,
        network: dfns_gen::generated::Network::Algorand,
        assets: Vec::new(),
    })
}
