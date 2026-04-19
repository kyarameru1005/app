use crate::errors::api_error::ApiError;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct MemoMessageResponse {
    pub message: &'static str,
}

pub async fn list_memos() -> Result<Json<MemoMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented("list memos is not implemented yet"))
}

pub async fn get_memo() -> Result<Json<MemoMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented("get memo is not implemented yet"))
}

pub async fn create_memo() -> Result<Json<MemoMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented("create memo is not implemented yet"))
}

pub async fn update_memo() -> Result<Json<MemoMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented("update memo is not implemented yet"))
}

pub async fn delete_memo() -> Result<Json<MemoMessageResponse>, ApiError> {
    Err(ApiError::NotImplemented("delete memo is not implemented yet"))
}
