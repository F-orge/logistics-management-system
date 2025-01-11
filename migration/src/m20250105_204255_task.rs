use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241228_193306_user::User, m20250105_194415_file::File};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TaskStatusEnum)
                    .values(TaskStatusVariant::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .col(
                        ColumnDef::new(Task::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Task::Title).varbit(255).not_null())
                    .col(ColumnDef::new(Task::Description).text().not_null())
                    .col(
                        ColumnDef::new(Task::Status)
                            .enumeration(Alias::new("task_status_enum"), TaskStatusVariant::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Task::IssuedByUser).uuid().not_null())
                    .col(ColumnDef::new(Task::Deadline).timestamp().not_null())
                    .col(
                        ColumnDef::new(Task::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Task::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Task::Table, Task::IssuedByUser)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Alias::new("task_assignee"))
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Alias::new("task_id")).uuid().not_null())
                    .col(ColumnDef::new(Alias::new("user_id")).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Alias::new("task_assignee"), Alias::new("task_id"))
                    .to(Task::Table, Task::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Alias::new("task_assignee"), Alias::new("user_id"))
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TaskComment::Table)
                    .col(
                        ColumnDef::new(TaskComment::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(TaskComment::TaskId).uuid().not_null())
                    .col(ColumnDef::new(TaskComment::UserId).uuid().not_null())
                    .col(ColumnDef::new(TaskComment::Comment).text().not_null())
                    .col(
                        ColumnDef::new(TaskComment::FileAttachment)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TaskComment::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TaskComment::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(TaskComment::Table, TaskComment::TaskId)
                    .to(Task::Table, Task::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(TaskComment::Table, TaskComment::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(TaskComment::Table, TaskComment::FileAttachment)
                    .to(File::Table, File::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskComment::Table).cascade().to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("task_assignee"))
                    .cascade()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Task::Table).cascade().to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TaskStatusEnum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    Title,
    Description,
    Status,
    IssuedByUser,
    Deadline,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub struct TaskStatusEnum;

#[derive(DeriveIden, EnumIter)]
pub enum TaskStatusVariant {
    Unassigned,
    InProgress,
    Review,
    Done,
}

#[derive(DeriveIden)]
pub enum TaskComment {
    Table,
    Id,
    TaskId,
    UserId,
    Comment,
    FileAttachment,
    CreatedAt,
    UpdatedAt,
}
