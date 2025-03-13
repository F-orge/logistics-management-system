use lib_core::AppState;
use utoipa_axum::router::OpenApiRouter;

pub mod shipment;
pub mod warehouse;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/shipment", shipment::routes())
        .nest("/warehouse", warehouse::routes())
}
