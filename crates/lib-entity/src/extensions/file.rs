use crate::generated::file::*;
use sea_orm::{
    ActiveModelBehavior, IntoActiveModel, Set,
    prelude::{Uuid, async_trait::async_trait},
    sqlx::types::chrono::Local,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PatchFileRequest {
    pub name: Option<String>,
    pub is_public: Option<bool>,
    pub shared_to: Option<Vec<Uuid>>,
}

impl IntoActiveModel<ActiveModel> for PatchFileRequest {
    fn into_active_model(self) -> ActiveModel {
        let mut active_model = ActiveModel::new();

        if let Some(name) = self.name {
            active_model.name = Set(name);
        }

        if let Some(is_public) = self.is_public {
            active_model.is_public = Set(is_public);
        }

        if let Some(shared_to) = self.shared_to {
            active_model.shared_to = Set(shared_to);
        }

        active_model
    }
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
        Ok(self)
    }
}
