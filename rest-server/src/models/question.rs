use serde::Serialize;
use sqlx::{Error, Row};
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct Question {
    content: String,
    options: Vec<String>,
    tag: i16,
    year: i16,
    origin: i16
}

#[derive(Serialize)]
pub struct Comment {
    content: String,
    user: UserComment,
    likes: i64,
    liked: bool
}

impl<'r> sqlx::FromRow<'r, PgRow> for Comment {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        Ok(
            Self {
                content: row.get("content"),
                user: UserComment {
                    // Uuid is not serializable
                    id: row.get::<Uuid, _>("user_id").to_string(),
                    username: row.get("user_username"),
                    avatar: row.get("user_avatar")
                },
                likes: row.get("likes"),
                liked: row.get("liked")
            }
        )
    }
}

#[derive(Serialize)]
struct UserComment {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>
}