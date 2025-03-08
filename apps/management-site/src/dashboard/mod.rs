use axum::Router;
use lib_core::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
}
