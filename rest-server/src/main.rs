use crate::web::App;

mod api;
mod models;
mod json;
mod web;

#[tokio::main]
async fn main() {
    App::new("application.toml")
        .await
        .run()
        .await;
}