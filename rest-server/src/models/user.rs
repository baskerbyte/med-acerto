use serde::Serialize;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct ProtectedUser {
    id: Uuid
}

#[derive(sqlx::FromRow, Serialize)]
pub struct PublicUser {
    pub username: String,
}