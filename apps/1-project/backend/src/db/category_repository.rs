use crate::models::category::Category;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        SELECT category_id, category_name
        FROM categories
        ORDER BY category_name ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_category_by_id(pool: &PgPool, category_id: Uuid) -> Result<Option<Category>, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        SELECT category_id, category_name
        FROM categories
        WHERE category_id = $1
        "#,
    )
    .bind(category_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_category(pool: &PgPool, category_name: &str) -> Result<Category, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (category_name)
        VALUES ($1)
        RETURNING category_id, category_name
        "#,
    )
    .bind(category_name)
    .fetch_one(pool)
    .await
}

pub async fn update_category(
    pool: &PgPool,
    category_id: Uuid,
    category_name: &str,
) -> Result<Option<Category>, sqlx::Error> {
    sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories
        SET category_name = $1
        WHERE category_id = $2
        RETURNING category_id, category_name
        "#,
    )
    .bind(category_name)
    .bind(category_id)
    .fetch_optional(pool)
    .await
}
