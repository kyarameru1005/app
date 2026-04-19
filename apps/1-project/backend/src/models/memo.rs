use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memo {
    pub memo_id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub user_id: Uuid,
    pub category_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
