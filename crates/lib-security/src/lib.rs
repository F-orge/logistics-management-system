use axum::{async_trait, extract::FromRequestParts};
use jwt::VerifyWithKey;
use lib_core::{AppState, error::Error};
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// reference: https://auth0.com/docs/secure/tokens/json-web-tokens/json-web-token-claims
#[derive(Deserialize, Serialize, Debug)]
pub struct JWTClaim {
    #[serde(rename = "iss")]
    pub issuer: String,
    #[serde(rename = "sub")]
    pub subject: sqlx::types::Uuid,
    #[serde(rename = "aud")]
    pub audience: String,
    #[serde(rename = "exp")]
    pub expiration: String,
    #[serde(rename = "nbf")]
    pub not_before: String,
    #[serde(rename = "iat")]
    pub issued_at: String,
    #[serde(rename = "jti")]
    pub jwt_id: sqlx::types::Uuid,
    #[serde(rename = "claims")]
    pub claims: BTreeMap<String, String>,
}

// TODO: implement a extractor for jwtclaims
#[async_trait]
impl<S> FromRequestParts<S> for JWTClaim {
    type Rejection = lib_core::error::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = parts
            .extensions
            .get::<AppState>()
            .ok_or(Error::AuthenticationError)?;

        let token = parts
            .headers
            .get("Authorization")
            .ok_or(Error::AuthenticationError)?
            .to_str()
            .map_err(|_| Error::AuthenticationError)?;

        let claims: JWTClaim = token
            .verify_with_key(&state.key)
            .map_err(|_| Error::AuthenticationError)?;

        Ok(claims)
    }
}
