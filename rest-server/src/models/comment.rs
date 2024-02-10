use serde::Serialize;
use sqlx::{Error, Row};
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;

#[derive(Serialize)]
pub struct Comment {
    content: String,
    user: UserComment,
    likes: i64,
    liked: bool
}

#[derive(Serialize)]
struct UserComment {
    id: String,
    username: String,
    avatar: Option<String>
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