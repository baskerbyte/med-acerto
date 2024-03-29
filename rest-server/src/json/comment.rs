use garde::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
pub struct CreateCommentPayload {
    #[garde(length(min = 5, max = 512))]
    pub content: String
}

#[derive(Deserialize, Validate)]
pub struct UpdateCommentPayload {
    #[garde(length(min = 5, max = 512))]
    pub content: String
}