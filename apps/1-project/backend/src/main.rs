use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct HealthResponse {
    message: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind backend server");
    axum::serve(listener, app)
        .await
        .expect("backend server failed");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        message: "backend is running",
    })
}
