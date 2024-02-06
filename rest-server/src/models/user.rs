use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
}