use axum::{Json, extract::State};
use tracing::info;

use crate::{
    dto::{ShortenRequest, ShortenResponse},
    services,
    state::AppState,
    store::UrlStore,
};

pub async fn shorten_handler(
    State(state): State<AppState>,
    Json(body): Json<ShortenRequest>,
) -> Json<ShortenResponse> {
    let url_original = body.url_original;
    let url_short = services::generate_url_short(&url_original);

    let mut store = state.store.lock().unwrap();
    store.insert(url_original.clone(), url_short.clone());

    info!(%url_short, %url_original, "created short url");

    Json(ShortenResponse {
        url_short,
        url_original,
    })
}
