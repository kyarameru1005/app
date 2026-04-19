use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub message: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        message: "backend is running",
    })
}
