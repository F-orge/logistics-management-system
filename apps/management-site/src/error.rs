use axum::{http::StatusCode, response::Html};
use lib_core::error::{AskamaError, FullError};

pub async fn not_found() -> AskamaError {
    lib_core::error::AskamaError::FullError(FullError {
        code: StatusCode::NOT_FOUND.as_u16(),
        message: "Page Not found".into(),
    })
}
