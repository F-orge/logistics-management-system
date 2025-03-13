use lib_core::AppState;
use utoipa_axum::{router::OpenApiRouter, routes};

#[utoipa::path(post, tag = "Warehouse Management", path = "/")]
async fn create() {}

#[utoipa::path(get, tag = "Warehouse Management", path = "/")]
async fn read() {}

#[utoipa::path(get, tag = "Warehouse Management", path = "/search")]
async fn search() {}

#[utoipa::path(get, tag = "Warehouse Management", path = "/{id}")]
async fn one() {}

#[utoipa::path(patch, tag = "Warehouse Management", path = "/{id}")]
async fn update() {}

#[utoipa::path(delete, tag = "Warehouse Management", path = "/{id}")]
async fn remove() {}

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create))
        .routes(routes!(read))
        .routes(routes!(search))
        .routes(routes!(one))
        .routes(routes!(update))
        .routes(routes!(remove))
}
