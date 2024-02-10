use serde::Serialize;
use sqlx::{Error, Row};
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow)]
pub struct ProtectedUser {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct PublicUser {
    id: String,
    username: String,
}

impl<'r> sqlx::FromRow<'r, PgRow> for PublicUser {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(
            Self {
                id: row.get::<Uuid, _>("id").to_string(),
                username: row.get("username")
            }
        )
    }
}