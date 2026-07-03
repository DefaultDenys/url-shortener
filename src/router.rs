use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

use crate::{handlers, state::AppState};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::index_handler))
        .route("/health", get(handlers::health_check_handler))
        .route("/shorten", post(handlers::shorten_handler))
        .route("/stats/{url_short}", get(handlers::stats_handler))
        .route("/{url_short}", get(handlers::redirect_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
