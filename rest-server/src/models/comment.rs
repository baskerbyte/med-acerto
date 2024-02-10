use serde::Serialize;
use sqlx::{Error, Row};
use sqlx::postgres::PgRow;

#[derive(Serialize)]
pub struct Comment {
    content: String,
    user: UserComment,
    likes: i64,
    liked: bool,
    created_at: i64,
    was_edited: bool
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
                    id: row.get("user_id"),
                    username: row.get("user_username"),
                    avatar: row.get("user_avatar")
                },
                likes: row.get("likes"),
                liked: row.get("liked"),
                created_at: row.get("created_at"),
                was_edited: row.get("was_edited"),
            }
        )
    }
}