mod get;

use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get::question_statics))
}