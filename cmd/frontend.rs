use axum::Router;
use base::Extension;
use crate_proto::auth::auth_service_client::AuthServiceClient;
use std::process::exit;
use tokio::net::TcpListener;
use tonic::transport::Channel;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    setup_tracing();

    let channel = match Channel::from_static("http://0.0.0.0:8081").connect().await {
        Ok(channel) => channel,
        Err(err) => {
            tracing::error!("{}", err);
            exit(1);
        }
    };

    // extensions for the system.
    let app: Router = axum::Router::new()
        .nest(
            "/auth",
            frontend_authentication::AuthenticationExtension {
                grpc_client: AuthServiceClient::new(channel.clone()),
                destination_url: "/".into(),
                action_url: "/auth/login".into(),
            }
            .router(),
        )
        .nest_service("/assets", ServeDir::new("dist"));

    // TODO: implement extension based system here.

    let listener = match TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!("{}", err);
            exit(1)
        }
    };

    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("{}", err)
    }
}
