use axum::Router;
use axum::routing::get;

mod comments;
mod get;
mod statics;

pub fn router() -> Router {
    Router::new()
        .nest("/:question_id/comments", comments::router())
        .nest("/:question_id/statics", statics::router())
        .route("/", get(get::filter_questions))
}