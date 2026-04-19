mod auth_routes;
mod category_routes;
mod memo_routes;

use crate::handlers::health_handler::health;
use crate::AppState;
use axum::{routing::get, Router};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .merge(auth_routes::routes())
        .merge(category_routes::routes())
        .merge(memo_routes::routes())
        .with_state(state)
}
