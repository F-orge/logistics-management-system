#![deny(clippy::unwrap_used)]

use std::{process::exit, sync::Arc, time::Duration};

use axum::{http::header, Router};
use controllers::{auth::AuthService, user::UserService};
use sea_orm::Database;
use tokio::net::TcpListener;
use tonic::transport::Server;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    decompression::DecompressionLayer,
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    sensitive_headers::{SetSensitiveHeadersLayer, SetSensitiveResponseHeadersLayer},
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    // Check if we are in debug mode
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.development").ok();
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    } else {
        dotenv::from_filename(".env.production").ok();
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    // Database connection
    tracing::debug!("Connecting to database");

    let db_url = std::env::var("DATABASE_URL").unwrap();

    let db = match Database::connect(db_url).await {
        Ok(value) => value,
        Err(_) => exit(1),
    };

    tracing::debug!("Connected to database");

    // App address and port
    let app_address = std::env::var("APP_ADDRESS").unwrap_or("127.0.0.1".into());
    let app_port = std::env::var("APP_PORT").unwrap_or("8080".into());

    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .expose_headers(Any)
        .allow_headers(Any);

    let headers = Arc::new([header::AUTHORIZATION, header::COOKIE, header::SET_COOKIE]);

    tracing::debug!("Setting up file service");

    let file_service = match cfg!(debug_assertions) {
        true => ServeDir::new("./target/release/frontend-build")
            .not_found_service(ServeFile::new("./target/release/frontedn-build/index.html")),
        false => ServeDir::new("./frontend-build")
            .not_found_service(ServeFile::new("./frontend-build/index.html")),
    };

    tracing::debug!("Setting up grpc service router");

    let grpc_server = Server::builder()
        // TODO: convert this "Authencation service" to a environment variable to hide it in the source code
        .add_service(AuthService::new(&db, "api.f-org-e.systems".into()))
        .add_service(UserService::new(&db))
        .into_service()
        .into_axum_router();

    // App routes
    let app: Router = Router::new()
        .nest("/grpc", grpc_server)
        .fallback_service(file_service)
        .layer(CatchPanicLayer::new())
        .layer(DecompressionLayer::new())
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(4096))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(SetSensitiveHeadersLayer::from_shared(headers.clone()))
        .layer(SetSensitiveResponseHeadersLayer::from_shared(headers))
        .layer(TimeoutLayer::new(Duration::from_secs(60)))
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Nanos),
                ),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Nanos),
                ),
        )
        .layer(cors);

    tracing::info!("Listening to: {} {}", app_address, app_port);

    // Start the server
    let listener = TcpListener::bind(format!("{}:{}", app_address, app_port))
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}
