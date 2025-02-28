#![deny(clippy::unwrap_used)]

use hmac::Hmac;
use sea_orm::{Database, DatabaseConnection};
use sha2::digest::KeyInit;
use tokio::{fs, net::TcpListener};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{path::Path, process::exit};

use tonic::transport::Server;

fn setup_tracing() {
    dotenv::dotenv().ok();
    if cfg!(debug_assertions) {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();

    tracing::info!("Starting Tonic server...");

    let db_url = std::env::var("RUST_DATABASE_URL")?;

    let jwt_key = Hmac::new_from_slice(std::env::var("RUST_JWT_SECRET")?.as_bytes())?;

    let directory = std::env::var("RUST_STORAGE_DIRECTORY_URL")?;

    let db = Database::connect(&db_url).await?;

    let grpc_server = Server::builder()
        .add_service(service_authentication::AuthService::new(
            &db,
            jwt_key.clone(),
        ))
        .add_service(service_storage::StorageService::new(
            &db,
            Path::new(&directory),
            jwt_key,
        ))
        .into_service()
        .into_axum_router();

    let app = grpc_server;

    let host = std::env::var("RUST_ADDRESS")?;
    let port = std::env::var("RUST_PORT")?;

    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
