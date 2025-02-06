#![deny(clippy::unwrap_used)]

use tokio::{fs, net::TcpListener};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::{path::Path, process::exit, sync::Arc, time::Duration};

use axum::{Router, http::header};
use tonic::transport::Server;
use tower_http::{
    LatencyUnit,
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    decompression::DecompressionLayer,
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    sensitive_headers::{SetSensitiveHeadersLayer, SetSensitiveResponseHeadersLayer},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

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

fn setup_layers(router: Router) -> Router {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .expose_headers(Any)
        .allow_headers(Any);

    let headers = Arc::new([header::AUTHORIZATION]);

    router
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

    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(err) => {
            tracing::error!("{}: DATABASE_URL", err);
            exit(1);
        }
    };

    let directory = match std::env::var("STORAGE_DIRECTORY_URL") {
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
            tracing::error!("{}: STORAGE_DIRECTORY_URL", err);
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

    let grpc_server = Server::builder()
        .add_service(authentication::AuthService::new(&db))
        .add_service(storage::StorageService::new(&db, Path::new(&directory)))
        .into_service()
        .into_axum_router();

    let app: Router = Router::new().nest("/grpc", grpc_server);

    let app = setup_layers(app);

    let host = match std::env::var("APP_ADDRESS") {
        Ok(host) => host,
        Err(err) => {
            tracing::error!("{}: ADDRESS", err);
            exit(1);
        }
    };

    let port = match std::env::var("APP_PORT") {
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

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("{}", err);
        exit(1);
    }
}
