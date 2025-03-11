use axum::{
    Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use derive_more::From;
use rinja::Template;
use serde::{Deserialize, Serialize};
use sqlx::types::uuid;

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
    // -- Authentication Error
    AuthenticationError,
    // -- Authorization
    AuthorizationError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

pub type Result<T> = core::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Json(ErrorResponse {
            code: 500,
            message: "Internal server error".into(),
        })
        .into_response()
    }
}
