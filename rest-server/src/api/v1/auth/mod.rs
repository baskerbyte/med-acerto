mod get;
mod post;

use axum::Router;
use axum::routing::{get, post};

pub fn router() -> Router {
    Router::new()
        .route("/logout", get(get::logout))
        .route("/login", post(post::login))
        .route("/sign-in", post(post::sign_in))
}