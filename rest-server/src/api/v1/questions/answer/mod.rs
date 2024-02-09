mod post;

use axum::Router;
use axum::routing::post;

pub fn router() -> Router {
    Router::new()
        .route("/", post(post::answer_question))
}