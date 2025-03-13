use axum::{Json, response::IntoResponse};
use derive_more::From;
use serde::{Deserialize, Serialize};
use sqlx::types::uuid;
use utoipa::ToSchema;

#[derive(From, Debug)]
pub enum Error {
    // -- Unhandled Error
    Custom(Box<dyn std::error::Error + Send + Sync>),
    // -- Database Error
    SeaOrm(sea_orm::DbErr),
    RowNotFound,
    // -- Validation
    Garde(garde::Report),
    // -- File Io
    Io(std::io::Error),
    // -- Validation Error,
    Uuid(uuid::Error),
    // -- Authentication
    AuthenticationError,
    // -- Authorization
    AuthorizationError,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = 400)]
    code: u16,
    #[schema(example = "Bad Request")]
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Json(ErrorResponse {
            code: 500,
            message: "Internal server error".into(),
        })
        .into_response()
    }
}
