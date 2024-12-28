use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.development").ok();
    } else {
        dotenv::from_filename(".env.production").ok();
    }

    cli::run_cli(migration::Migrator).await;
}
