use crate::auth::session;
use crate::db::category_repository;
use crate::errors::api_error::ApiError;
use crate::AppState;
use axum::{extract::Path, extract::State, Json};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CategoryResponse {
    pub category_id: String,
    pub category_name: String,
}

#[derive(Serialize)]
pub struct CategoryListResponse {
    pub categories: Vec<CategoryResponse>,
}

#[derive(Serialize)]
pub struct CategoryMessageResponse {
    pub category_id: String,
    pub message: &'static str,
}

#[derive(Deserialize)]
pub struct CategoryRequest {
    pub category_name: String,
}

pub async fn list_categories(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<CategoryListResponse>, ApiError> {
    require_login(&jar)?;
    let categories = category_repository::list_categories(&state.pool).await?;

    Ok(Json(CategoryListResponse {
        categories: categories
            .into_iter()
            .map(|row| CategoryResponse {
                category_id: row.category_id.to_string(),
                category_name: row.category_name,
            })
            .collect(),
    }))
}

pub async fn create_category(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<CategoryRequest>,
) -> Result<Json<CategoryMessageResponse>, ApiError> {
    require_login(&jar)?;
    validate_category_name(&payload.category_name)?;

    let result = category_repository::create_category(&state.pool, payload.category_name.trim())
        .await
        .map_err(map_category_error)?;
    Ok(Json(CategoryMessageResponse {
        category_id: result.category_id.to_string(),
        message: "カテゴリを登録しました",
    }))
}

pub async fn update_category(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(category_id): Path<Uuid>,
    Json(payload): Json<CategoryRequest>,
) -> Result<Json<CategoryMessageResponse>, ApiError> {
    require_login(&jar)?;
    validate_category_name(&payload.category_name)?;

    let category = category_repository::update_category(
        &state.pool,
        category_id,
        payload.category_name.trim(),
    )
    .await
    .map_err(map_category_error)?;

    let category = category.ok_or_else(|| ApiError::NotFound("category not found".to_string()))?;
    Ok(Json(CategoryMessageResponse {
        category_id: category.category_id.to_string(),
        message: "カテゴリを更新しました",
    }))
}

fn require_login(jar: &CookieJar) -> Result<(), ApiError> {
    session::read_session_user(jar)
        .map(|_| ())
        .ok_or(ApiError::Unauthorized)
}

fn validate_category_name(category_name: &str) -> Result<(), ApiError> {
    let trimmed = category_name.trim();
    if trimmed.is_empty() {
        return Err(ApiError::BadRequest(
            "カテゴリ名を入力してください".to_string(),
        ));
    }
    if trimmed.chars().count() > 100 {
        return Err(ApiError::BadRequest(
            "カテゴリ名は100文字以内で入力してください".to_string(),
        ));
    }
    Ok(())
}

fn map_category_error(err: sqlx::Error) -> ApiError {
    if let sqlx::Error::Database(db_err) = &err {
        if db_err.code().as_deref() == Some("23505") {
            return ApiError::Conflict("同じカテゴリ名が既に存在します".to_string());
        }
    }
    ApiError::from(err)
}
