use hmac::Hmac;
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use tonic::{Status, metadata::MetadataMap};

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

pub fn get_jwt_claim(
    metadata: &MetadataMap,
    encryption_key: &Hmac<Sha256>,
) -> Result<JWTClaim, Status> {
    let authorization = metadata
        .get("authorization")
        .ok_or(Status::unauthenticated(
            "Cannot get `authorization` header metadata",
        ))?
        .clone();

    let (auth_type, token) = {
        let auth_str = authorization
            .to_str()
            .map_err(|_| Status::unauthenticated("Cannot get `authorization` header metadata"))?;
        let mut parts = auth_str.splitn(2, ' ');
        let auth_type = parts.next().ok_or(Status::unauthenticated(
            "Cannot get `authorization` header metadata",
        ))?;
        let token = parts.next().ok_or(Status::unauthenticated(
            "Cannot get `authorization` header metadata",
        ))?;
        (auth_type, token)
    };

    if auth_type.to_lowercase() != "bearer" {
        return Err(Status::unauthenticated(
            "Cannot get `authorization` header metadata",
        ));
    }

    let claims: JWTClaim = token
        .verify_with_key(encryption_key)
        .map_err(|_| Status::unauthenticated("Cannot get `authorization` header metadata"))?;

    Ok(claims)
}
