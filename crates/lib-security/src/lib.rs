use axum::RequestPartsExt;
use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use jwt::VerifyWithKey;
use lib_core::{AppState, error::Error};
use sea_orm::prelude::DateTimeUtc;
use sqlx::types::chrono::Local;
use std::collections::BTreeMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};

// reference: https://auth0.com/docs/secure/tokens/json-web-tokens/json-web-token-claims
#[derive(Deserialize, Serialize, Debug, Clone)]
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
    pub claims: BTreeMap<String, Vec<Permission>>,
}

impl<S> FromRequestParts<S> for JWTClaim
where
    AppState: FromRef<S>,
    S: Send + Sync + Debug,
{
    type Rejection = lib_core::error::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = parts
            .extract_with_state::<AppState, _>(state)
            .await
            .map_err(|_| Error::AuthenticationError)?;

        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(Error::AuthenticationError)?
            .to_str()
            .map_err(|_| Error::AuthenticationError)?;

        let (format, token) = auth_header
            .split_once(' ')
            .ok_or(Error::AuthenticationError)?;

        if format.to_lowercase() != "bearer" {
            return Err(Error::AuthenticationError);
        }

        let claims: JWTClaim = token
            .verify_with_key(&state.key)
            .map_err(|_| Error::AuthenticationError)?;

        let exp: DateTimeUtc = claims
            .expiration
            .parse()
            .map_err(|_| Error::AuthenticationError)?;

        let nbf: DateTimeUtc = claims
            .not_before
            .parse()
            .map_err(|_| Error::AuthenticationError)?;

        if exp < Local::now() {
            return Err(Error::AuthenticationError);
        }

        if nbf > Local::now() {
            return Err(Error::AuthenticationError);
        }

        Ok(claims)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Update,
    Delete,
    Bypass,
}

impl Into<sea_orm::Value> for Permission {
    fn into(self) -> sea_orm::Value {
        match self {
            Permission::Read => "read".into(),
            Permission::Write => "write".into(),
            Permission::Update => "update".into(),
            Permission::Delete => "delete".into(),
            Permission::Bypass => "bypass".into(),
        }
    }
}

impl Into<String> for Permission {
    fn into(self) -> String {
        match self {
            Permission::Read => "read".into(),
            Permission::Write => "write".into(),
            Permission::Update => "update".into(),
            Permission::Delete => "delete".into(),
            Permission::Bypass => "bypass".into(),
        }
    }
}

impl TryInto<Permission> for String {
    type Error = sea_orm::DbErr;
    fn try_into(self) -> Result<Permission, Self::Error> {
        match self.as_str() {
            "read" => Ok(Permission::Read),
            "write" => Ok(Permission::Write),
            "update" => Ok(Permission::Update),
            "delete" => Ok(Permission::Delete),
            "bypass" => Ok(Permission::Bypass),
            _ => Err(sea_orm::DbErr::AttrNotSet("cannot set permission".into())),
        }
    }
}

pub fn verify_permission(
    claims: &JWTClaim,
    table: &str,
    permissions: Vec<Permission>,
) -> lib_core::result::Result<()> {
    let perm_claim: &Vec<Permission> = claims.claims.get(table).ok_or(Error::AuthorizationError)?;

    if !perm_claim.iter().any(|perm| permissions.contains(perm)) {
        return Err(Error::AuthorizationError);
    }

    Ok(())
}
