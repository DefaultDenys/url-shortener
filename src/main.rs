mod dto;
mod entities;
mod handlers;
mod router;
mod services;
mod state;
mod store;
mod tracing_config;

use std::{env, net::SocketAddr};

use migration::{Migrator, MigratorTrait};
use state::AppState;

use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_config::init_tracing();

    let db = store::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    Migrator::up(&db, None).await.unwrap();

    let url_repository = store::UrlRepository::new(db);

    let state = AppState::new(url_repository);

    let app = router::build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!(%addr, "server listening");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
