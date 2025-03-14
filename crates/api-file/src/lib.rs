use lib_core::{AppState, error::ErrorResponse};
use serde::Deserialize;
use utoipa::{ToSchema};
use lib_entity::generated::file;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Debug, ToSchema, Deserialize)]
struct UploadFileDTO {
    #[schema(format = Binary, content_media_type = "application/octet-stream")]
    file: String,
}

#[utoipa::path(
    post,
    operation_id = "UploadFile",
    request_body(content = UploadFileDTO, content_type = "multipart/form-data"),
    tag = "File Management",
    path = "/upload"
)]
async fn upload() {}

#[utoipa::path(
    get,
    operation_id = "DownloadFile",
    tag = "File Management",
    path = "/download",
    responses(
        (status = 200, content_type = "application/octet-stream"),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn download() {}

#[utoipa::path(
    get, 
    tag = "File Management", 
    path = "/",
    responses(
        (status = 200, body = Vec<file::Model>),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn read() {}

#[utoipa::path(
    get, 
    tag = "File Management", 
    path = "/search",
    responses(
        (status = 200, body = Vec<file::Model>),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn search() {}

#[utoipa::path(
    get, 
    tag = "File Management", 
    path = "/{id}",
    responses(
        (status = 200, body = file::Model),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn one() {}

#[utoipa::path(
    patch, 
    tag = "File Management", 
    path = "/{id}",
    responses(
        (status = 200, body = file::Model),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn update() {}

#[utoipa::path(
    delete, 
    tag = "File Management", 
    path = "/{id}",
    responses(
        (status = 204),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn remove() {}

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(upload))
        .routes(routes!(download))
        .routes(routes!(read))
        .routes(routes!(search))
        .routes(routes!(one))
        .routes(routes!(update))
        .routes(routes!(remove))
}
