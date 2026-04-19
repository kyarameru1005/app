use crate::errors::api_error::ApiError;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: &'static str,
}

pub async fn start_google_login() -> Result<Json<MessageResponse>, ApiError> {
    Err(ApiError::NotImplemented(
        "google login start is not implemented yet",
    ))
}

pub async fn google_login_callback() -> Result<Json<MessageResponse>, ApiError> {
    Err(ApiError::NotImplemented(
        "google login callback is not implemented yet",
    ))
}

pub async fn logout() -> Result<Json<MessageResponse>, ApiError> {
    Err(ApiError::NotImplemented("logout is not implemented yet"))
}

pub async fn get_me() -> Result<Json<MessageResponse>, ApiError> {
    Err(ApiError::NotImplemented("get me is not implemented yet"))
}
