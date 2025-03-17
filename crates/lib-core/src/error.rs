use axum::{Json, http::StatusCode, response::IntoResponse};
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
    pub code: u16,
    #[schema(example = "Bad Request")]
    pub message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("{:#?}", self);
        let (response, status) = match self {
            Error::AuthenticationError => (
                ErrorResponse {
                    code: 401,
                    message: "Authentication Error".into(),
                },
                StatusCode::UNAUTHORIZED,
            ),
            Error::AuthorizationError => (
                ErrorResponse {
                    code: 403,
                    message: "Authorization Error".into(),
                },
                StatusCode::FORBIDDEN,
            ),
            Error::RowNotFound => (
                ErrorResponse {
                    code: 404,
                    message: "Row not found".into(),
                },
                StatusCode::NOT_FOUND,
            ),
            _ => (
                ErrorResponse {
                    code: 500,
                    message: "Internal server error".into(),
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        let mut response = Json(response).into_response();

        *response.status_mut() = status;

        response
    }
}
