use serde::Serialize;

#[derive(sqlx::FromRow)]
pub struct ProtectedUser {}

#[derive(sqlx::FromRow, Serialize)]
pub struct PublicUser {
    pub username: String,
}