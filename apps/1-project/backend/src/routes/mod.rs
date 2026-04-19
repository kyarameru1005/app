mod auth_routes;
mod category_routes;
mod memo_routes;

use crate::handlers::health_handler::health;
use crate::AppState;
use axum::http::{header, HeaderValue, Method};
use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().expect("valid origin"));

    Router::new()
        .route("/health", get(health))
        .merge(auth_routes::routes())
        .merge(category_routes::routes())
        .merge(memo_routes::routes())
        .layer(cors)
        .with_state(state)
}
