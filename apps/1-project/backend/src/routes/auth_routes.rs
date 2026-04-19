use crate::handlers::auth_handler;
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/google/login", post(auth_handler::start_google_login))
        .route(
            "/api/auth/google/callback",
            get(auth_handler::google_login_callback),
        )
        .route("/api/auth/logout", post(auth_handler::logout))
        .route("/api/users/me", get(auth_handler::get_me))
}
