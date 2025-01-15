#![deny(clippy::unwrap_used)]

use std::{process::exit, sync::Arc, time::Duration};

use axum::{http::header, Router};
use controllers::{auth::AuthService, user::UserService};
use hmac::{Hmac, Mac};
use sea_orm::{Database, DatabaseConnection};
use sha2::Sha256;
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
    set_status::SetStatus,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod controllers;
mod models;
mod utils;

#[derive(Clone)]
pub struct AppState {
    key: Hmac<Sha256>,
    db: DatabaseConnection,
}

fn setup_tracing() {
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
}

fn setup_file_service() -> ServeDir<SetStatus<ServeFile>> {
    match cfg!(debug_assertions) {
        true => ServeDir::new("./target/release/frontend-build")
            .not_found_service(ServeFile::new("./target/release/frontedn-build/index.html")),
        false => ServeDir::new("./frontend-build")
            .not_found_service(ServeFile::new("./frontend-build/index.html")),
    }
}

fn setup_address_and_port() -> (String, String) {
    let app_address = std::env::var("APP_ADDRESS").unwrap_or("127.0.0.1".into());
    let app_port = std::env::var("APP_PORT").unwrap_or("8080".into());
    (app_address, app_port)
}

fn setup_layers(router: Router, app_state: AppState) -> Router {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .expose_headers(Any)
        .allow_headers(Any);

    let headers = Arc::new([header::AUTHORIZATION, header::COOKIE, header::SET_COOKIE]);

    router
        .layer(axum::middleware::from_fn_with_state(
            app_state,
            controllers::auth::auth_middleware,
        ))
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
        .layer(cors)
}

#[tokio::main]
async fn main() {
    setup_tracing();

    // Database connection
    tracing::debug!("Connecting to database");

    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => {
            tracing::error!("Unable to get `DATABASE_URL` at environment variables.");
            exit(1);
        }
    };

    let db = match Database::connect(db_url).await {
        Ok(value) => value,
        Err(_) => exit(1),
    };

    tracing::debug!("Connected to database");

    let key: Hmac<Sha256> = match Hmac::new_from_slice(b"some-random-key") {
        Ok(value) => value,
        Err(err) => {
            tracing::error!("encryption key error: {}", err);
            exit(1);
        }
    };

    let app_state = AppState { key, db };

    // App address and port
    let (address, port) = setup_address_and_port();

    tracing::debug!("Setting up file service");

    let file_service = setup_file_service();

    tracing::debug!("Setting up grpc service router");

    let grpc_server = Server::builder()
        // TODO: convert this "Authencation service" to a environment variable to hide it in the source code
        .add_service(AuthService::new(
            &app_state.db,
            "api.f-org-e.systems".into(),
        ))
        .add_service(UserService::new(&app_state.db))
        .into_service()
        .into_axum_router();

    // App routes
    let app: Router = Router::new()
        .nest("/grpc", grpc_server)
        .fallback_service(file_service);

    let app = setup_layers(app, app_state);

    tracing::info!("Listening to: {} {}", address, port);

    // Start the server
    let listener = TcpListener::bind(format!("{}:{}", address, port))
        .await
        .expect("Failed to bind address");

    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("Unable to serve application");
            exit(1)
        }
    };
}
