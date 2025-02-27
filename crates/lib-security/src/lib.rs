// reference: https://auth0.com/docs/secure/tokens/json-web-tokens/json-web-token-claims
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

use axum::{async_trait, extract::FromRequestParts, http::StatusCode};
use serde::{Deserialize, Serialize};

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

#[async_trait]
impl<S> FromRequestParts<S> for JWTClaim {
    type Rejection = StatusCode;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let key = parts
            .extensions
            .get::<Hmac<Sha256>>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        // get the authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::FORBIDDEN)?;

        let token = auth_header
            .to_str()
            .map_err(|_| StatusCode::FORBIDDEN)?
            .to_string();

        let claims: Self = token
            .verify_with_key(key)
            .map_err(|_| StatusCode::FORBIDDEN)?;

        Ok(claims)
    }
}
