use crate::handlers::category_handler;
use crate::AppState;
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/categories", get(category_handler::list_categories))
        .route("/api/categories", post(category_handler::create_category))
        .route(
            "/api/categories/:category_id",
            put(category_handler::update_category),
        )
}
