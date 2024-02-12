use serde_json::json;
use sqlx::types::chrono::NaiveDateTime;

pub struct Comment {
    pub content: String,
    pub likes: Option<i64>,
    pub liked: Option<bool>,
    pub created_at: NaiveDateTime,
    pub was_edited: bool,
    pub user_id: i32,
    pub user_username: String,
    pub user_avatar: Option<String>
}

impl Comment {
    pub fn into_json(&self) -> serde_json::Value {
        json!({
            "content": self.content,
            "likes": self.likes,
            "liked": self.liked,
            "created_at": self.created_at.timestamp(),
            "was_edited": self.was_edited,
            "user": {
                "id": self.user_id,
                "username": self.user_username,
                "avatar": self.user_avatar,
            }
        })
    }
}