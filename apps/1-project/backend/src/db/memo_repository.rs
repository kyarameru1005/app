use crate::models::memo::Memo;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_memos_by_user(_pool: &PgPool, _user_id: Uuid) -> Result<Vec<Memo>, sqlx::Error> {
    Ok(vec![])
}

pub async fn find_memo_by_id(_pool: &PgPool, _memo_id: Uuid) -> Result<Option<Memo>, sqlx::Error> {
    Ok(None)
}
