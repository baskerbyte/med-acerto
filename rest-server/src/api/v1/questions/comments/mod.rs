use axum::Router;
use axum::routing::get;

mod get;
mod post;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get::question_comments)
            .post(post::create_comment)
        )
}
