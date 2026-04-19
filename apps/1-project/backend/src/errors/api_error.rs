use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found: {0}")]
    NotFound(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    #[error("internal server error")]
    InternalServerError,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match self {
            ApiError::BadRequest(message) => {
                if message.is_empty() {
                    "入力内容を確認してください".to_string()
                } else {
                    message
                }
            }
            ApiError::Unauthorized => "ログインが必要です".to_string(),
            ApiError::Forbidden => "この操作を行う権限がありません".to_string(),
            ApiError::NotFound(message) => {
                if message.is_empty() {
                    "対象のデータが存在しません".to_string()
                } else {
                    message
                }
            }
            ApiError::Conflict(message) => {
                if message.is_empty() {
                    "同じデータが既に存在します".to_string()
                } else {
                    message
                }
            }
            ApiError::NotImplemented(_) | ApiError::InternalServerError => {
                "サーバエラーが発生しました".to_string()
            }
        };

        let body = ErrorResponse {
            message,
        };

        (status, Json(body)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(_value: sqlx::Error) -> Self {
        ApiError::InternalServerError
    }
}
