use lib_core::AppState;
use utoipa_axum::router::OpenApiRouter;

pub mod employee;
pub mod task;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/task", task::routes())
        .nest("/employee", employee::routes())
}
