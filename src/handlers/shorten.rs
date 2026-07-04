use axum::{Json, extract::State, http::StatusCode};
use tracing::info;

use crate::{
    dto::{ShortenRequest, ShortenResponse},
    services,
    state::AppState,
};

pub async fn shorten_handler(
    State(state): State<AppState>,
    Json(body): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, StatusCode> {
    let url_original = body.url_original;
    let url_repository = state.url_repository;

    let existing_url = url_repository.find_by_original_url(&url_original).await;

    if let Some(url_model) = existing_url {
        tracing::debug!(url_original = %url_model.url_original, "reused existing short url");
        return Ok(Json(ShortenResponse {
            url_short: url_model.url_short,
            url_original: url_model.url_original,
            click_count: url_model.click_count,
        }));
    }

    let url_short = services::generate_url_short();

    let db_result = url_repository
        .insert(url_original, url_short)
        .await
        .map_err(|err| {
            tracing::error!(%err, "failed to insert url");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(%db_result.url_short, %db_result.url_original, "created short url");
    Ok(Json(ShortenResponse {
        url_short: db_result.url_short,
        url_original: db_result.url_original,
        click_count: db_result.click_count,
    }))
}