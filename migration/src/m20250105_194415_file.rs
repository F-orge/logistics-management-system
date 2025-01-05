use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241228_193306_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .col(
                        ColumnDef::new(File::Id)
                            .uuid()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(File::Name)
                            .varbit(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(File::Path).text().not_null().unique_key())
                    .col(ColumnDef::new(File::OwnerId).uuid().not_null())
                    .col(
                        ColumnDef::new(File::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(File::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(File::Table, File::OwnerId)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum File {
    Table,
    Id,
    Name,
    Path,
    OwnerId,
    CreatedAt,
    UpdatedAt,
}
