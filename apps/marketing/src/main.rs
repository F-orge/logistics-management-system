use axum::Router;
use include_dir::{Dir, include_dir};
use tokio::net::TcpListener;
use tower_serve_static::ServeDir;

mod landing;

static ASSET_DIR: Dir<'static> = include_dir!("public");
static FLYONUI_DIR: Dir<'static> = include_dir!("./node_modules/flyonui/dist/js/");

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    let assets_service = ServeDir::new(&ASSET_DIR);
    let js_assets_service = ServeDir::new(&FLYONUI_DIR);

    let router = Router::new()
        .nest("/", landing::routes())
        .nest_service("/assets", assets_service)
        .nest_service("/assets/js", js_assets_service);

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
