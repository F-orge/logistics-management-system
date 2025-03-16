use std::collections::BTreeMap;

use lib_core::error::Error;
use lib_security::Permission;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, IntoActiveModel, QueryFilter, Set, TransactionTrait, prelude::Uuid,
};

use crate::generated::permissions;

pub mod file;
pub mod users;

pub async fn grant_permission<C: ConnectionTrait>(
    db: &C,
    user_id: Uuid,
    table: &str,
    permissions: Vec<Permission>,
) -> Result<(), sea_orm::DbErr> {
    for perm in permissions.into_iter() {
        let model = crate::generated::prelude::Permissions::find()
            .filter(permissions::Column::UserId.eq(user_id))
            .filter(permissions::Column::Action.eq(perm.clone()))
            .filter(permissions::Column::EntityName.eq(table))
            .one(db)
            .await?;

        if model.is_some() {
            continue;
        }

        let mut model = permissions::ActiveModel::new();
        model.entity_name = Set(table.to_string());
        model.user_id = Set(user_id);
        model.action = Set(perm.into());
        _ = model.insert(db).await?;
    }

    Ok(())
}

pub async fn revoke_permission(
    db: &DatabaseConnection,
    user_id: Uuid,
    table: &str,
    permissions: Vec<Permission>,
) -> lib_core::result::Result<()> {
    let trx = db.begin().await.map_err(Error::SeaOrm)?;
    for perm in permissions.into_iter() {
        let model = crate::generated::prelude::Permissions::find()
            .filter(permissions::Column::UserId.eq(user_id))
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

pub async fn get_all_permissions(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<BTreeMap<String, Vec<Permission>>, sea_orm::DbErr> {
    let user_perms = permissions::Entity::find()
        .filter(permissions::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    let mut map: BTreeMap<String, Vec<Permission>> = BTreeMap::new();

    for perm in user_perms {
        let action: Permission = perm.action.try_into()?;
        // create a new vec if entry doesnt exist. else append the action
        map.entry(perm.entity_name)
            .or_insert_with(Vec::new)
            .push(action);
    }

    Ok(map)
}
