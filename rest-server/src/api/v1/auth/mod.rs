mod get;
mod post;
pub mod gate;

use axum::Router;
use axum::routing::{get, post};
use crate::api::v1::auth::gate::protected;

pub fn router() -> Router {
    Router::new()
        .route("/logout", get(get::logout))
        .route_layer(axum::middleware::from_fn(protected))
        .route("/login", post(post::login))
        .route("/sign-in", post(post::sign_in))
}