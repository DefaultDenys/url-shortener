use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use tracing::info;

use crate::state::AppState;

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(url_short): Path<String>,
) -> Result<Redirect, StatusCode> {
    let url_repository = state.url_repository;
    let search_result = url_repository.find_by_short_url(&url_short).await;

    match search_result {
        Some(url_model) => {
            info!(%url_model.url_short, %url_model.url_original, "redirecting to original url");
            if let Err(err) = url_repository.increment_click_count(&url_short).await {
                tracing::error!(%err, "failed to increment click count");
            }
            Ok(Redirect::temporary(&url_model.url_original))
        }
        None => {
            tracing::warn!(%url_short, "short url not found");
            Err(StatusCode::NOT_FOUND)
        }
    }
}
