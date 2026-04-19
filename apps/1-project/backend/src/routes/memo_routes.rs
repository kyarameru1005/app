use crate::handlers::memo_handler;
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/memos", get(memo_handler::list_memos))
        .route("/api/memos", post(memo_handler::create_memo))
        .route("/api/memos/:memo_id", get(memo_handler::get_memo))
        .route("/api/memos/:memo_id", put(memo_handler::update_memo))
        .route("/api/memos/:memo_id", delete(memo_handler::delete_memo))
}
