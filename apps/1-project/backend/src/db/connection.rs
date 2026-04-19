use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

fn env_or_default(key: &str, default_value: &str) -> String {
    env::var(key).unwrap_or_else(|_| default_value.to_string())
}

pub fn database_url_from_env() -> String {
    let host = env_or_default("DB_HOST", "localhost");
    let port = env_or_default("DB_PORT", "5432");
    let db_name = env_or_default("DB_NAME", "memo_app");
    let user = env_or_default("DB_USER", "postgres");
    let password = env_or_default("DB_PASSWORD", "postgres");

    format!("postgres://{user}:{password}@{host}:{port}/{db_name}")
}

pub async fn create_pool_from_env() -> Result<PgPool, sqlx::Error> {
    let database_url = database_url_from_env();
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}
