mod store;

use axum::routing::post;
use axum::{Json, Router, extract::State, routing::get};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use store::{InMemoryStore, UrlStore};

use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() {
    let mut store = InMemoryStore::new();
    // Add a sample URL mapping to the store
    store.insert("https://www.rust-lang.org".to_string(), "rust".to_string());

    let state = AppState {
        store: Arc::new(Mutex::new(store)),
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/shorten", post(shorten_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler(State(state): State<AppState>) -> String {
    let store = state.store.lock().unwrap();
    format!(
        "{} URL(s) stored. Lookup ('rust') = {:?}",
        store.count(),
        store.lookup("rust")
    )
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

    Json(ShortenResponse {
        url_short,
        url_original,
    })
}
