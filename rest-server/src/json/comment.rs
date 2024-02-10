use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCommentPayload {
    pub content: String
}