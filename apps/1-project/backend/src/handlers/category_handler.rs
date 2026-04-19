use crate::errors::api_error::ApiError;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct CategoryMessageResponse {
    pub message: &'static str,
}

pub async fn list_categories() -> Result<Json<CategoryMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented(
        "list categories is not implemented yet",
    ))
}

pub async fn create_category() -> Result<Json<CategoryMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented(
        "create category is not implemented yet",
    ))
}

pub async fn update_category() -> Result<Json<CategoryMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented(
        "update category is not implemented yet",
    ))
}
