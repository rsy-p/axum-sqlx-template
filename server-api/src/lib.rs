use axum::routing::get;
use axum::Router;
use server_core::AppState;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(server_service::find_by_id))
        .with_state(state.clone())
}
