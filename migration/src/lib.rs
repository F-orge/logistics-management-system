pub use sea_orm_migration::prelude::*;

mod m20241228_193306_user;
mod m20250105_194415_file;
mod m20250105_200657_employee;
mod m20250105_204255_task;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241228_193306_user::Migration),
            Box::new(m20250105_194415_file::Migration),
            Box::new(m20250105_200657_employee::Migration),
            Box::new(m20250105_204255_task::Migration),
        ]
    }
}
