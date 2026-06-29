use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};
use tracing::info;

use crate::{state::AppState, store::UrlStore};

pub async fn redirect_handler(
    State(state): State<AppState>,
    Path(url_short): Path<String>,
) -> Result<Redirect, StatusCode> {
    let store = state.store.lock().unwrap();

    match store.lookup(&url_short) {
        Some(url_original) => {
            info!(%url_short, %url_original, "redirecting to original url");
            Ok(Redirect::temporary(&url_original))
        }
        None => {
            tracing::warn!(%url_short, "short url not found");
            Err(StatusCode::NOT_FOUND)
        }
    }
}
