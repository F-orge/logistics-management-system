#![deny(clippy::unwrap_used)]

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
async fn main() {
    println!("Strarting!");
    setup_tracing();

    let db_url = match std::env::var("RUST_DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(err) => {
            tracing::error!("{}: RUST_DATABASE_URL", err);
            exit(1);
        }
    };

    let jwt_key = match std::env::var("RUST_JWT_SECRET") {
        Ok(jwt_key) => jwt_key,
        Err(err) => {
            tracing::error!("{}: RUST_JWT_SECRET", err);
            exit(1);
        }
    };

    let directory = match std::env::var("RUST_STORAGE_DIRECTORY_URL") {
        Ok(directory) => {
            let path = Path::new(&directory);
            if !path.exists() {
                tracing::warn!("{} does not exist. Creating...", directory);
                if let Err(err) = fs::create_dir(path).await {
                    tracing::error!("{}", err);
                    exit(1);
                }
            }
            directory
        }
        Err(err) => {
            tracing::error!("{}: RUST_STORAGE_DIRECTORY_URL", err);
            exit(1);
        }
    };

    let db = match sqlx::PgPool::connect(&db_url).await {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("{}", err);
            exit(1);
        }
    };

    let _ = match sqlx::query("SELECT set_config('app.jwt_secret', $1,false)")
        .bind(jwt_key)
        .execute(&db)
        .await
    {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("{}", err);
            exit(1)
        }
    };

    let grpc_server = Server::builder()
        .add_service(service_authentication::AuthService::new(&db))
        .add_service(service_storage::StorageService::new(
            &db,
            Path::new(&directory),
        ))
        .into_service()
        .into_axum_router();

    let app = grpc_server;

    let host = match std::env::var("RUST_ADDRESS") {
        Ok(host) => host,
        Err(err) => {
            tracing::error!("{}: ADDRESS", err);
            exit(1);
        }
    };

    let port = match std::env::var("RUST_PORT") {
        Ok(port) => port,
        Err(err) => {
            tracing::error!("{}: PORT", err);
            exit(1);
        }
    };

    let listener = match TcpListener::bind(format!("{}:{}", host, port)).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!("{}", err);
            exit(1);
        }
    };

    if let Err(err) = axum::serve(listener, app.into_make_service()).await {
        tracing::error!("{}", err);
        exit(1);
    }
}
