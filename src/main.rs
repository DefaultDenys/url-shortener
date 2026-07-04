mod dto;
mod entities;
mod handlers;
mod rate_limit;
mod router;
mod services;
mod state;
mod store;
mod tracing_config;

use std::{env, net::SocketAddr, path::Path};

use migration::{Migrator, MigratorTrait};
use state::AppState;

use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_config::init_tracing();

    let database_url = database_url();
    ensure_sqlite_parent_dir(&database_url);
    info!("connecting to database");

    let db = store::connect(&database_url)
        .await
        .expect("failed to connect to database — check DATABASE_URL");

    Migrator::up(&db, None)
        .await
        .expect("failed to run migrations");

    let url_repository = store::UrlRepository::new(db.clone());
    let click_repository = store::ClickRepository::new(db);

    let state = AppState::new(url_repository, click_repository);

    let app = router::build_router(state);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse().expect("PORT must be a number")));
    info!(%addr, "server listening");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://data.db?mode=rwc".to_owned())
}

fn ensure_sqlite_parent_dir(database_url: &str) {
    if !database_url.starts_with("sqlite:") {
        return;
    }

    let path_part = database_url
        .trim_start_matches("sqlite://")
        .trim_start_matches("sqlite:")
        .split('?')
        .next()
        .unwrap_or("");

    if path_part.is_empty() {
        return;
    }

    if let Some(parent) = Path::new(path_part).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("failed to create SQLite directory");
        }
    }
}