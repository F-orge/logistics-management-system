use axum::Router;
use hmac::{Hmac, Mac};
use lib_core::AppState;
use sea_orm::Database;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::connect(std::env::var("RUST_DATABASE_URL")?).await?;

    let (port, address) = (std::env::var("RUST_PORT")?, std::env::var("RUST_ADDRESS")?);

    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    let (router, openapi) = OpenApiRouter::with_openapi(lib_core::docs::OpenAPI::openapi())
        .nest("/api/v1/auth", api_auth::routes())
        .nest("/api/v1/human-resource", api_human_resource::routes())
        .nest("/api/v1/inventory", api_inventory::routes())
        .nest("/api/v1/file", api_file::routes())
        .nest("/api/v1/chat", api_chat::routes())
        .nest("/api/v1/sales", api_sales::routes())
        .split_for_parts();

    let router = router
        .merge(Scalar::with_url("/scalar", openapi))
        .with_state(AppState {
            db,
            key: Hmac::new_from_slice(std::env::var("RUST_JWT_ACCESS_KEY")?.as_bytes())?,
        });

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
