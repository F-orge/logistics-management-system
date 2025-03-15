use std::{fmt::Debug, path::PathBuf};

use axum::{
    extract::{FromRef, FromRequestParts},
    http::StatusCode,
};
use hmac::Hmac;
use sea_orm::DatabaseConnection;
use sha2::Sha256;

pub mod error;
pub mod result;
// pub mod test;
pub mod docs;
pub mod middleware;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub key: Hmac<Sha256>,
    pub storage_path: PathBuf,
}

impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync + Debug,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
