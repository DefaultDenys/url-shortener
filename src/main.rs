mod store;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
    routing::{get, post},
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use store::{InMemoryStore, UrlStore};

use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    store: Arc<Mutex<InMemoryStore>>,
}

#[derive(Deserialize)]
struct ShortenRequest {
    url_original: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    url_short: String,
    url_original: String,
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "url_shortener=debug,tower_http=debug,axum::rejection=trace".into()
        }))
        .init();
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let mut store = InMemoryStore::new();
    // Add a sample URL mapping to the store
    store.insert("https://www.rust-lang.org".to_string(), "rust".to_string());

    let state = AppState {
        store: Arc::new(Mutex::new(store)),
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_check_handler))
        .route("/shorten", post(shorten_handler))
        .route("/{url_short}", get(redirect_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!(%addr, "server listening");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check_handler() -> String {
    format!("Ok")
}

async fn index_handler() -> Html<&'static str> {
    tracing::debug!("serving index page");
    Html(include_str!("../templates/index.html"))
}

fn generate_url_short(url_original: &str) -> String {
    let mut hasher = DefaultHasher::new();
    url_original.hash(&mut hasher);
    format!("{:06x}", hasher.finish() & 0xFFFFFF)
}

async fn shorten_handler(
    State(state): State<AppState>,
    Json(body): Json<ShortenRequest>,
) -> Json<ShortenResponse> {
    let url_original = body.url_original;
    let url_short = generate_url_short(&url_original);

    let mut store = state.store.lock().unwrap();
    store.insert(url_original.clone(), url_short.clone());

    info!(%url_short, %url_original, "created short url");

    Json(ShortenResponse {
        url_short,
        url_original,
    })
}

async fn redirect_handler(
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
