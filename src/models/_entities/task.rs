//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use super::sea_orm_active_enums::TaskStatusEnum;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: Vec<u8>,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub status: TaskStatusEnum,
    pub issued_by_user: Uuid,
    pub deadline: DateTime,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::task_assignee::Entity")]
    TaskAssignee,
    #[sea_orm(has_many = "super::task_comment::Entity")]
    TaskComment,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::IssuedByUser",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::task_assignee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskAssignee.def()
    }
}

impl Related<super::task_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskComment.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
