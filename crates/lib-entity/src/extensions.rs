use sea_orm::{ActiveModelBehavior, Set, prelude::async_trait::async_trait};

use crate::{employee, users};

#[async_trait]
impl ActiveModelBehavior for employee::ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, sea_orm::DbErr>
    where
        C: sea_orm::ConnectionTrait,
    {
        if !insert {
            return Ok(self);
        }

        let mut user = users::ActiveModel::new();

        // TODO: create auto generated password and email if not exists

        Ok(self)
    }
}
