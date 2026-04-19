use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_user_by_google_sub(_pool: &PgPool, _google_sub: &str) -> Result<Option<User>, sqlx::Error> {
    Ok(None)
}

pub async fn find_user_by_id(_pool: &PgPool, _user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    Ok(None)
}
