//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "logistics", table_name = "department")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::department_employees::Entity")]
    DepartmentEmployees,
    #[sea_orm(has_many = "super::job_information::Entity")]
    JobInformation,
}

impl Related<super::department_employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DepartmentEmployees.def()
    }
}

impl Related<super::job_information::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobInformation.def()
    }
}

impl Related<super::employee::Entity> for Entity {
    fn to() -> RelationDef {
        super::department_employees::Relation::Employee.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::department_employees::Relation::Department
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
