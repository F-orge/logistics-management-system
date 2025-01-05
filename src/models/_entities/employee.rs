//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use super::sea_orm_active_enums::ContractTypeEnum;
use super::sea_orm_active_enums::SexEnum;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "employee")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: Vec<u8>,
    pub middle_name: Vec<u8>,
    pub last_name: Vec<u8>,
    pub sex: SexEnum,
    pub address: Vec<u8>,
    pub position: Vec<u8>,
    pub contact_number: Vec<u8>,
    pub contract_type: ContractTypeEnum,
    pub birthday: Date,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
