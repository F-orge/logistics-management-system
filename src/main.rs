#![deny(clippy::unwrap_used)]

use std::{sync::Arc, time::Duration};

use axum::{http::header, Router};
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

    // App address and port
    let app_address = std::env::var("APP_ADDRESS").expect("APP_ADDRESS not set");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT not set");

    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .expose_headers(Any)
        .allow_headers(Any);

    let headers = Arc::new([header::AUTHORIZATION, header::COOKIE, header::SET_COOKIE]);

    let file_service = match cfg!(debug_assertions) {
        true => ServeDir::new("./target/release/frontend-build")
            .not_found_service(ServeFile::new("./target/release/frontedn-build/index.html")),
        false => ServeDir::new("./frontend-build")
            .not_found_service(ServeFile::new("./frontend-build/index.html")),
    };

    // GRPC server
    /*
    let grpc_server = Server::builder()
        .into_service()
        .into_axum_router();
    */

    // App routes
    let app: Router = Router::new()
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

    // Start the server
    let listener = TcpListener::bind(format!("{}:{}", app_address, app_port))
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.unwrap();
}
