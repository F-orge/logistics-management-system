use pwhash::{bcrypt, sha256_crypt};
use sea_orm::{
    ActiveModelBehavior, DeriveIntoActiveModel, IntoActiveModel, IntoActiveValue, Set,
    prelude::async_trait::async_trait, sqlx::types::chrono::Local,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::generated::users::*;

#[derive(Debug, Deserialize, Serialize, DeriveIntoActiveModel, ToSchema)]
pub struct CreateUserRequest {
    pub auth_type: String,
    pub email: String,
    pub password: String,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, sea_orm::DbErr>
    where
        C: sea_orm::ConnectionTrait,
    {
        if !insert {
            self.updated = Set(Local::now().naive_utc());
        }
        if let Some(sea_orm::Value::String(Some(password))) = self.password.clone().into_value() {
            self.password = Set(bcrypt::hash(*password)
                .map_err(|_| sea_orm::DbErr::AttrNotSet("cannot hash password".into()))?);
        }
        Ok(self)
    }
}
