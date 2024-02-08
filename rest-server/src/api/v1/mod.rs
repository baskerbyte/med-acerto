use axum::Router;

pub mod auth;
mod users;
mod questions;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/questions", questions::router())
}