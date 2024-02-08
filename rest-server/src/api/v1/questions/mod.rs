use axum::Router;
use axum::routing::get;

mod comments;
mod get;

pub fn router() -> Router {
    Router::new()
        .nest("/:question_id/comments", comments::router())
        .route("/", get(get::filter_questions))
}