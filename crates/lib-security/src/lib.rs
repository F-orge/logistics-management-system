use axum::{async_trait, extract::FromRequestParts};
use jwt::VerifyWithKey;
use lib_core::{AppState, error::Error};
use lib_entity::permissions;
use sea_orm::ActiveModelBehavior;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::IntoActiveModel;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};
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

#[derive(Debug, Clone)]
pub enum Permission {
    Read,
    Write,
    Update,
    Delete,
}

impl Into<sea_orm::Value> for Permission {
    fn into(self) -> sea_orm::Value {
        match self {
            Permission::Read => "read".into(),
            Permission::Write => "write".into(),
            Permission::Update => "update".into(),
            Permission::Delete => "delete".into(),
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
        }
    }
}

pub async fn verify_permission(
    db: &DatabaseConnection,
    claims: &JWTClaim,
    table: &str,
    permissions: Vec<Permission>,
) -> lib_core::result::Result<bool> {
    Ok(lib_entity::prelude::Permissions::find()
        .filter(permissions::Column::EntityName.eq(table))
        .filter(permissions::Column::UserId.eq(claims.subject))
        .filter(permissions::Column::Action.is_in(permissions))
        .one(db)
        .await
        .map_err(Error::SeaOrm)?
        .is_some())
}

pub async fn grant_permission(
    db: &DatabaseConnection,
    claims: &JWTClaim,
    table: &str,
    permissions: Vec<Permission>,
) -> lib_core::result::Result<()> {
    let trx = db.begin().await.map_err(Error::SeaOrm)?;
    for perm in permissions.into_iter() {
        let model = lib_entity::prelude::Permissions::find()
            .filter(permissions::Column::UserId.eq(claims.subject))
            .filter(permissions::Column::Action.eq(perm.clone()))
            .filter(permissions::Column::EntityName.eq(table))
            .one(db)
            .await
            .map_err(Error::SeaOrm)?;

        if model.is_some() {
            continue;
        }

        let mut model = permissions::ActiveModel::new();
        model.entity_name = Set(table.to_string());
        model.user_id = Set(claims.subject);
        model.action = Set(perm.into());
        _ = model.insert(&trx).await.map_err(Error::SeaOrm)?;
    }

    trx.commit().await.map_err(Error::SeaOrm)?;

    Ok(())
}

pub async fn revoke_permission(
    db: &DatabaseConnection,
    claims: &JWTClaim,
    table: &str,
    permissions: Vec<Permission>,
) -> lib_core::result::Result<()> {
    let trx = db.begin().await.map_err(Error::SeaOrm)?;
    for perm in permissions.into_iter() {
        let model = lib_entity::prelude::Permissions::find()
            .filter(permissions::Column::UserId.eq(claims.subject))
            .filter(permissions::Column::Action.eq(perm))
            .filter(permissions::Column::EntityName.eq(table))
            .one(db)
            .await
            .map_err(Error::SeaOrm)?;

        if model.is_none() {
            continue;
        }

        let model = model.ok_or(Error::RowNotFound)?.into_active_model();

        _ = model.delete(&trx).await.map_err(Error::SeaOrm)?;
    }

    trx.commit().await.map_err(Error::SeaOrm)?;

    Ok(())
}
