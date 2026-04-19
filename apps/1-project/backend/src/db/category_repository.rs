use crate::models::category::Category;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_categories(_pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    Ok(vec![])
}

pub async fn find_category_by_id(_pool: &PgPool, _category_id: Uuid) -> Result<Option<Category>, sqlx::Error> {
    Ok(None)
}
