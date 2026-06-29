mod dto;
mod handlers;
mod router;
mod services;
mod state;
mod store;
mod tracing_config;

use std::net::SocketAddr;

use state::AppState;
use store::InMemoryStore;

use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_config::init_tracing();

    let store = InMemoryStore::new();
    let state = AppState::new(store);

    let app = router::build_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!(%addr, "server listening");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
