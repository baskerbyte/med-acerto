use axum::Router;

pub mod auth;
mod users;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
}