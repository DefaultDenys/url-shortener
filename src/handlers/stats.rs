use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{dto::StatsResponse, state::AppState};

pub async fn stats_handler(
    State(state): State<AppState>,
    Path(url_short): Path<String>,
) -> Result<Json<StatsResponse>, StatusCode> {
    let url_model = state
        .url_repository
        .find_by_short_url(&url_short)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let clicks_per_minute = state
        .click_repository
        .clicks_per_minute(&url_short)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let clicks_per_hour = state
        .click_repository
        .clicks_per_hour(&url_short)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let clicks_per_day = state
        .click_repository
        .clicks_per_day(&url_short)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(StatsResponse {
        url_short: url_model.url_short,
        url_original: url_model.url_original,
        click_count: url_model.click_count,
        created_at: url_model.created_at,
        clicks_per_minute,
        clicks_per_hour,
        clicks_per_day,
    }))
}