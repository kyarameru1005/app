use crate::auth::{google, session};
use crate::db::user_repository;
use crate::errors::api_error::ApiError;
use crate::AppState;
use axum::{extract::Query, extract::State, Json};
use axum_extra::extract::cookie::CookieJar;
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct LoginStartResponse {
    pub message: &'static str,
    pub auth_url: &'static str,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub user_id: String,
    pub google_sub: String,
    pub user_name: String,
    pub email: String,
}

#[derive(serde::Deserialize)]
pub struct GoogleCallbackQuery {
    pub google_sub: String,
    pub user_name: String,
    pub email: String,
}

pub async fn start_google_login() -> Result<Json<LoginStartResponse>, ApiError> {
    Ok(Json(LoginStartResponse {
        message: "Googleログインを開始します",
        auth_url: google::login_url(),
    }))
}

pub async fn google_login_callback(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<GoogleCallbackQuery>,
) -> Result<(CookieJar, Json<MessageResponse>), ApiError> {
    if query.google_sub.trim().is_empty() || query.user_name.trim().is_empty() || query.email.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "google_sub, user_name, email are required".to_string(),
        ));
    }

    let user = user_repository::upsert_user_by_google_sub(
        &state.pool,
        &query.google_sub,
        &query.user_name,
        &query.email,
    )
    .await?;

    let jar = jar.add(session::create_session_cookie(user.user_id));
    Ok((
        jar,
        Json(MessageResponse {
            message: "ログインしました",
        }),
    ))
}

pub async fn logout(jar: CookieJar) -> Result<(CookieJar, Json<MessageResponse>), ApiError> {
    let jar = jar.remove(session::clear_session_cookie());
    Ok((
        jar,
        Json(MessageResponse {
            message: "ログアウトしました",
        }),
    ))
}

pub async fn get_me(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<MeResponse>, ApiError> {
    let session_user = session::read_session_user(&jar).ok_or(ApiError::Unauthorized)?;
    let user = user_repository::find_user_by_id(&state.pool, session_user.user_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("user not found".to_string()))?;

    Ok(Json(MeResponse {
        user_id: user.user_id.to_string(),
        google_sub: user.google_sub,
        user_name: user.user_name,
        email: user.email,
    }))
}
