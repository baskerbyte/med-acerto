use axum::Router;

mod comments;

pub fn router() -> Router {
    Router::new()
        .nest("/:question_id/comments", comments::router())
}