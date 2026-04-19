mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod routes;

use crate::db::connection::create_pool_from_env;
use crate::routes::create_router;
use sqlx::PgPool;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pool = create_pool_from_env()
        .await
        .expect("failed to initialize db pool");
    let state = AppState { pool };
    let app = create_router(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend server");

    tracing::info!("backend listening on {}", addr);
    axum::serve(listener, app)
        .await
        .expect("backend server failed");
}
