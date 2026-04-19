use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SessionUser {
    pub user_id: Uuid,
}
