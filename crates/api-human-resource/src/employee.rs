use lib_core::{AppState,error::ErrorResponse};
use utoipa_axum::{router::OpenApiRouter, routes};
use lib_entity::generated::employee;
#[utoipa::path(
    post, 
    tag = "Employee Management", 
    path = "/",
    responses(
        (status = 200, body = employee::Model),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
async fn create() {}

#[utoipa::path(
    get, 
    tag = "Employee Management", 
    path = "/",
    responses(
        (status = 200, body = Vec<employee::Model>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
async fn read() {}

#[utoipa::path(
    get, 
    tag = "Employee Management", 
    path = "/search",
    responses(
        (status = 200, body = Vec<employee::Model>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
async fn search() {}

#[utoipa::path(
    get, 
    tag = "Employee Management", 
    path = "/{id}",
    responses(
        (status = 200, body = employee::Model),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
async fn one() {}

#[utoipa::path(
    patch, 
    tag = "Employee Management", 
    path = "/{id}",
    responses(
        (status = 200, body = employee::Model),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
async fn update() {}

#[utoipa::path(
    delete, 
    tag = "Employee Management", 
    path = "/{id}",
    responses(
        (status = 200, description = "Success" ),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
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
