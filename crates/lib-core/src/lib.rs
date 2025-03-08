use sea_orm::DatabaseConnection;

pub mod error;
pub mod streaming;
pub mod test;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
