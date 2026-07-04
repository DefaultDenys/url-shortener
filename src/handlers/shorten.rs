use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use tracing::info;

use crate::{
    dto::{ShortenRequest, ShortenResponse, bad_request, validate_original_url},
    services,
    state::AppState,
};

pub async fn shorten_handler(
    State(state): State<AppState>,
    Json(body): Json<ShortenRequest>,
) -> impl IntoResponse {
    let url_original = match validate_original_url(&body.url_original) {
        Ok(url) => url,
        Err(err) => {
            tracing::warn!(reason = err.message(), "invalid shorten request");
            return bad_request(err.message()).into_response();
        }
    };

    let url_repository = state.url_repository;

    let existing_url = url_repository.find_by_original_url(&url_original).await;

    if let Some(url_model) = existing_url {
        tracing::debug!(url_original = %url_model.url_original, "reused existing short url");
        return Json(ShortenResponse {
            url_short: url_model.url_short,
            url_original: url_model.url_original,
            click_count: url_model.click_count,
        })
        .into_response();
    }

    let url_short = services::generate_url_short();

    let db_result = match url_repository.insert(url_original, url_short).await {
        Ok(result) => result,
        Err(err) => {
            tracing::error!(%err, "failed to insert url");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    info!(%db_result.url_short, %db_result.url_original, "created short url");
    Json(ShortenResponse {
        url_short: db_result.url_short,
        url_original: db_result.url_original,
        click_count: db_result.click_count,
    })
    .into_response()
}