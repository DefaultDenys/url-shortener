use axum::{
    Router,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;

use crate::{handlers, rate_limit, state::AppState};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::index_handler))
        .route("/health", get(handlers::health_check_handler))
        .merge(
            Router::new()
                .route("/shorten", post(handlers::shorten_handler))
                .layer(rate_limit::shorten_layer()),
        )
        .merge(
            Router::new()
                .route("/stats/{url_short}", get(handlers::stats_handler))
                .layer(rate_limit::stats_layer()),
        )
        .merge(
            Router::new()
                .route("/{url_short}", get(handlers::redirect_handler))
                .layer(rate_limit::redirect_layer()),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}