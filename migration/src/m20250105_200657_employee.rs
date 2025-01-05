use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241228_193306_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(SexEnum)
                    .values(SexVariant::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ContractTypeEnum)
                    .values(ContractTypeVariant::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Employee::Table)
                    .col(
                        ColumnDef::new(Employee::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Employee::UserId).uuid().not_null())
                    .col(ColumnDef::new(Employee::FirstName).varbit(255).not_null())
                    .col(ColumnDef::new(Employee::MiddleName).varbit(255).not_null())
                    .col(ColumnDef::new(Employee::LastName).varbit(255).not_null())
                    .col(
                        ColumnDef::new(Employee::Sex)
                            .enumeration(Alias::new("sex_enum"), SexVariant::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Employee::Address).varbit(255).not_null())
                    .col(ColumnDef::new(Employee::Position).varbit(255).not_null())
                    .col(
                        ColumnDef::new(Employee::ContactNumber)
                            .varbit(15)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Employee::ContractType)
                            .enumeration(
                                Alias::new("contract_type_enum"),
                                ContractTypeVariant::iter(),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Employee::Birthday).date().not_null())
                    .col(
                        ColumnDef::new(Employee::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Employee::UpdatedAt)
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
                    .from(Employee::Table, Employee::UserId)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Employee::Table).cascade().to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(SexEnum).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(ContractTypeEnum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Employee {
    Table,
    Id,
    UserId,
    FirstName,
    MiddleName,
    LastName,
    Sex,
    Address,
    Position,
    ContactNumber,
    ContractType,
    Birthday,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub struct SexEnum;

#[derive(DeriveIden, EnumIter)]
pub enum SexVariant {
    Male,
    Female,
}

#[derive(DeriveIden)]
pub struct ContractTypeEnum;

#[derive(DeriveIden, EnumIter)]
pub enum ContractTypeVariant {
    FullTime,
    PartTime,
}
