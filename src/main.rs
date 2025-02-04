#![deny(clippy::unwrap_used)]

use std::{process::exit, sync::Arc, time::Duration};

use authentication::AuthService;
use axum::{Router, http::header};
use cli::CLI;
use sqlx::{Connection, Pool, Postgres};
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
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod cli;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
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

fn setup_layers(router: Router, app_state: AppState) -> Router {
    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .expose_headers(Any)
        .allow_headers(Any);

    let headers = Arc::new([header::AUTHORIZATION, header::COOKIE, header::SET_COOKIE]);

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

    // Database connection
    tracing::debug!("Connecting to database");

    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => {
            tracing::error!("Unable to get `DATABASE_URL` at environment variables.");
            exit(1);
        }
    };

    let db = match sqlx::PgPool::connect(&db_url).await {
        Ok(value) => value,
        Err(_) => exit(1),
    };

    tracing::debug!("Connected to database");

    let app_state = AppState { db };

    tracing::debug!("Setting up grpc service router");

    let grpc_server = Server::builder()
        // TODO: convert this "Authencation service" to a environment variable to hide it in the source code
        .add_service(AuthService::new(&app_state.db))
        .into_service()
        .into_axum_router();

    // App routes
    let app: Router = Router::new().nest("/grpc", grpc_server);

    let app = setup_layers(app, app_state);
    CLI::new()
        .about("CLI management tool")
        .serve(app)
        .start()
        .await;
}
