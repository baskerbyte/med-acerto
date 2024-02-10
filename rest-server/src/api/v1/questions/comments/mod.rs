use axum::Router;
use axum::routing::{delete, get};

mod get;
mod post;
mod delete;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get::question_comments)
            .post(post::create_comment)
        )
        .route("/:comment_id", delete(delete::delete_comment))
}
