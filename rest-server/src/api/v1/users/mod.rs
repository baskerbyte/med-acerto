use axum::Router;
use axum::routing::get;

mod get;

pub fn router() -> Router {
    Router::new()
        .route("/:id", get(get::user_info))
}