use crate::models::memo::Memo;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct MemoWithCategory {
    pub memo_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub category_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn list_memos_by_user(
    pool: &PgPool,
    user_id: Uuid,
    keyword: Option<&str>,
    category_id: Option<Uuid>,
) -> Result<Vec<MemoWithCategory>, sqlx::Error> {
    sqlx::query_as::<_, MemoWithCategory>(
        r#"
        SELECT
            n.memo_id,
            n.title,
            n.content,
            n.user_id,
            n.category_id,
            c.category_name,
            n.created_at,
            n.updated_at
        FROM notes n
        LEFT JOIN categories c ON c.category_id = n.category_id
        WHERE
            n.user_id = $1
            AND (
                $2::text IS NULL
                OR n.title ILIKE '%' || $2 || '%'
                OR n.content ILIKE '%' || $2 || '%'
            )
            AND ($3::uuid IS NULL OR n.category_id = $3)
        ORDER BY n.created_at DESC
        "#,
    )
    .bind(user_id)
    .bind(keyword)
    .bind(category_id)
    .fetch_all(pool)
    .await
}

pub async fn find_memo_by_id_for_user(
    pool: &PgPool,
    memo_id: Uuid,
    user_id: Uuid,
) -> Result<Option<MemoWithCategory>, sqlx::Error> {
    sqlx::query_as::<_, MemoWithCategory>(
        r#"
        SELECT
            n.memo_id,
            n.title,
            n.content,
            n.user_id,
            n.category_id,
            c.category_name,
            n.created_at,
            n.updated_at
        FROM notes n
        LEFT JOIN categories c ON c.category_id = n.category_id
        WHERE n.memo_id = $1 AND n.user_id = $2
        "#,
    )
    .bind(memo_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_memo(
    pool: &PgPool,
    title: &str,
    content: Option<&str>,
    user_id: Uuid,
    category_id: Option<Uuid>,
) -> Result<Memo, sqlx::Error> {
    sqlx::query_as::<_, Memo>(
        r#"
        INSERT INTO notes (title, content, user_id, category_id)
        VALUES ($1, $2, $3, $4)
        RETURNING memo_id, title, content, user_id, category_id, created_at, updated_at
        "#,
    )
    .bind(title)
    .bind(content)
    .bind(user_id)
    .bind(category_id)
    .fetch_one(pool)
    .await
}

pub async fn update_memo_for_user(
    pool: &PgPool,
    memo_id: Uuid,
    user_id: Uuid,
    title: &str,
    content: Option<&str>,
    category_id: Option<Uuid>,
) -> Result<Option<Memo>, sqlx::Error> {
    sqlx::query_as::<_, Memo>(
        r#"
        UPDATE notes
        SET title = $1, content = $2, category_id = $3, updated_at = NOW()
        WHERE memo_id = $4 AND user_id = $5
        RETURNING memo_id, title, content, user_id, category_id, created_at, updated_at
        "#,
    )
    .bind(title)
    .bind(content)
    .bind(category_id)
    .bind(memo_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn delete_memo_for_user(
    pool: &PgPool,
    memo_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM notes
        WHERE memo_id = $1 AND user_id = $2
        "#,
    )
    .bind(memo_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
