use sea_orm::{DeriveIntoActiveModel, IntoActiveModel, IntoActiveValue, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::generated::{sea_orm_active_enums::AuthType, users::*};

#[derive(Debug, Deserialize, Serialize, DeriveIntoActiveModel, ToSchema)]
pub struct CreateUserDTO {
    pub auth_type: AuthType,
    pub email: String,
    pub password: String,
}

impl IntoActiveValue<AuthType> for AuthType {
    fn into_active_value(self) -> sea_orm::ActiveValue<AuthType> {
        match self {
            AuthType::BasicAuth => Set(AuthType::BasicAuth),
        }
    }
}
