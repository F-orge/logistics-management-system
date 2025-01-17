//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(
    schema_name = "etmar_logistics",
    table_name = "task_comment_file_attachment"
)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub file_id: Uuid,
    pub task_comment_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::FileId",
        to = "super::file::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    File,
    #[sea_orm(
        belongs_to = "super::task_comment::Entity",
        from = "Column::TaskCommentId",
        to = "super::task_comment::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TaskComment,
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

impl Related<super::task_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TaskComment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
