use crate::auth::session;
use crate::db::{category_repository, memo_repository};
use crate::errors::api_error::ApiError;
use crate::AppState;
use axum::{extract::Path, extract::Query, extract::State, Json};
use axum_extra::extract::cookie::CookieJar;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct MemoResponse {
    pub memo_id: String,
    pub title: String,
    pub content: String,
    pub category_id: Option<String>,
    pub category_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct MemoListResponse {
    pub memos: Vec<MemoResponse>,
}

#[derive(Serialize)]
pub struct MemoMessageResponse {
    pub memo_id: String,
    pub message: &'static str,
}

#[derive(Deserialize)]
pub struct MemoQuery {
    pub keyword: Option<String>,
    pub category_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct MemoRequest {
    pub title: String,
    pub content: Option<String>,
    pub category_id: Option<Uuid>,
}

pub async fn list_memos(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<MemoQuery>,
) -> Result<Json<MemoListResponse>, ApiError> {
    let session_user = require_login(&jar)?;
    let memos = memo_repository::list_memos_by_user(
        &state.pool,
        session_user.user_id,
        query.keyword.as_deref(),
        query.category_id,
    )
    .await?;

    Ok(Json(MemoListResponse {
        memos: memos.into_iter().map(to_memo_response).collect(),
    }))
}

pub async fn get_memo(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(memo_id): Path<Uuid>,
) -> Result<Json<MemoResponse>, ApiError> {
    let session_user = require_login(&jar)?;
    let memo = memo_repository::find_memo_by_id_for_user(&state.pool, memo_id, session_user.user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("対象のメモが存在しません".to_string()))?;
    Ok(Json(to_memo_response(memo)))
}

pub async fn create_memo(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<MemoRequest>,
) -> Result<Json<MemoMessageResponse>, ApiError> {
    let session_user = require_login(&jar)?;
    validate_memo_request(&payload)?;
    ensure_category_exists_if_needed(&state, payload.category_id).await?;

    let memo = memo_repository::create_memo(
        &state.pool,
        payload.title.trim(),
        payload.content.as_deref(),
        session_user.user_id,
        payload.category_id,
    )
    .await?;

    Ok(Json(MemoMessageResponse {
        memo_id: memo.memo_id.to_string(),
        message: "メモを登録しました",
    }))
}

pub async fn update_memo(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(memo_id): Path<Uuid>,
    Json(payload): Json<MemoRequest>,
) -> Result<Json<MemoMessageResponse>, ApiError> {
    let session_user = require_login(&jar)?;
    validate_memo_request(&payload)?;
    ensure_category_exists_if_needed(&state, payload.category_id).await?;

    let memo = memo_repository::update_memo_for_user(
        &state.pool,
        memo_id,
        session_user.user_id,
        payload.title.trim(),
        payload.content.as_deref(),
        payload.category_id,
    )
    .await?
    .ok_or_else(|| ApiError::NotFound("対象のメモが存在しません".to_string()))?;

    Ok(Json(MemoMessageResponse {
        memo_id: memo.memo_id.to_string(),
        message: "メモを更新しました",
    }))
}

pub async fn delete_memo(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(memo_id): Path<Uuid>,
) -> Result<Json<MemoMessageResponse>, ApiError> {
    let session_user = require_login(&jar)?;
    let deleted = memo_repository::delete_memo_for_user(&state.pool, memo_id, session_user.user_id)
        .await?;
    if !deleted {
        return Err(ApiError::NotFound("対象のメモが存在しません".to_string()));
    }
    Ok(Json(MemoMessageResponse {
        memo_id: memo_id.to_string(),
        message: "メモを削除しました",
    }))
}

fn require_login(jar: &CookieJar) -> Result<session::SessionUser, ApiError> {
    session::read_session_user(jar).ok_or(ApiError::Unauthorized)
}

fn validate_memo_request(payload: &MemoRequest) -> Result<(), ApiError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(ApiError::BadRequest(
            "タイトルを入力してください".to_string(),
        ));
    }
    if title.chars().count() > 200 {
        return Err(ApiError::BadRequest(
            "タイトルは200文字以内で入力してください".to_string(),
        ));
    }
    Ok(())
}

async fn ensure_category_exists_if_needed(
    state: &AppState,
    category_id: Option<Uuid>,
) -> Result<(), ApiError> {
    if let Some(id) = category_id {
        let exists = category_repository::find_category_by_id(&state.pool, id).await?;
        if exists.is_none() {
            return Err(ApiError::NotFound("対象のカテゴリが存在しません".to_string()));
        }
    }
    Ok(())
}

fn to_memo_response(row: memo_repository::MemoWithCategory) -> MemoResponse {
    MemoResponse {
        memo_id: row.memo_id.to_string(),
        title: row.title,
        content: row.content.unwrap_or_default(),
        category_id: row.category_id.map(|id| id.to_string()),
        category_name: row.category_name.unwrap_or_else(|| "未分類".to_string()),
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}
