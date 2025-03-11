use axum::{Router, http::StatusCode};
use error::not_found;
use include_dir::{Dir, include_dir};
use lib_core::{AppState, error::FullError};
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tower_serve_static::ServeDir;

mod dashboard;
mod error;
mod forgot_password;
mod login;

static ASSET_DIR: Dir<'static> = include_dir!("public");
static FLYONUI_DIR: Dir<'static> = include_dir!("./node_modules/flyonui/");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:3001").await?;

    let assets_service = ServeDir::new(&ASSET_DIR);
    let js_assets_service = ServeDir::new(&FLYONUI_DIR);

    let db =
        Database::connect("postgres://postgres:randompassword@localhost:5432/postgres").await?;

    let router = Router::new()
        .nest("/", dashboard::routes())
        .nest("/login", login::routes())
        .nest("/forgot-password", forgot_password::routes())
        .nest_service("/assets", assets_service)
        .nest_service("/assets/js", js_assets_service)
        .fallback(not_found)
        .with_state(AppState { db });

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
