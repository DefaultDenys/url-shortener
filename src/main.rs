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

    let database_url = database_url();
    info!("connecting to database");

    let db = store::connect(&database_url)
        .await
        .expect("failed to connect to database — check DATABASE_URL and that PostgreSQL is linked");

    Migrator::up(&db, None)
        .await
        .expect("failed to run migrations");

    let url_repository = store::UrlRepository::new(db);

    let state = AppState::new(url_repository);

    let app = router::build_router(state);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().expect("PORT must be a number")));
    info!(%addr, "server listening");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn database_url() -> String {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("DATABASE_PRIVATE_URL"))
        .expect("DATABASE_URL must be set — on Railway: add PostgreSQL and link it to this service");

    // Railway public Postgres proxy requires TLS with rustls
    if url.contains("rlwy.net") && !url.contains("sslmode=") {
        if url.contains('?') {
            format!("{url}&sslmode=require")
        } else {
            format!("{url}?sslmode=require")
        }
    } else {
        url
    }
}
