use hmac::Hmac;
use sea_orm::DatabaseConnection;
use sha2::Sha256;

pub mod error;
pub mod result;
// pub mod test;
pub mod middleware;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub key: Hmac<Sha256>,
}
