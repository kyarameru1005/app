use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn find_user_by_google_sub(pool: &PgPool, google_sub: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT user_id, google_sub, user_name, email, created_at, updated_at
        FROM users
        WHERE google_sub = $1
        "#,
    )
    .bind(google_sub)
    .fetch_optional(pool)
    .await
}

pub async fn find_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT user_id, google_sub, user_name, email, created_at, updated_at
        FROM users
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn upsert_user_by_google_sub(
    pool: &PgPool,
    google_sub: &str,
    user_name: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (google_sub, user_name, email)
        VALUES ($1, $2, $3)
        ON CONFLICT (google_sub)
        DO UPDATE SET
            user_name = EXCLUDED.user_name,
            email = EXCLUDED.email,
            updated_at = NOW()
        RETURNING user_id, google_sub, user_name, email, created_at, updated_at
        "#,
    )
    .bind(google_sub)
    .bind(user_name)
    .bind(email)
    .fetch_one(pool)
    .await
}
