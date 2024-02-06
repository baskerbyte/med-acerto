use axum::Router;

pub mod auth;

pub fn router() -> Router {
    Router::new()
        .merge(auth::router())
}