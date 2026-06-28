mod store;

use axum::{Router, extract::State, routing::get};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use store::{InMemoryStore, UrlStore};

#[derive(Clone)]
struct AppState {
    store: Arc<Mutex<InMemoryStore>>,
}

#[tokio::main]
async fn main() {
    let mut store = InMemoryStore::new();
    // Add a sample URL mapping to the store
    store.insert("https://www.rust-lang.org".to_string(), "rust".to_string());

    let state = AppState {
        store: Arc::new(Mutex::new(store)),
    };

    let app = Router::new().route("/", get(root_handler).with_state(state));

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
