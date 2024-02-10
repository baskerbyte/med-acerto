use serde::Serialize;

#[derive(sqlx::FromRow)]
pub struct ProtectedUser {
    pub id: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct PublicUser {
    id: i64,
    username: String,
}