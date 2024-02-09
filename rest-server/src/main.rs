mod api;
mod models;
mod json;

use axum::{Extension, Router};
use http::{header, Method};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use common::settings::AppSettings;
use app_core::database::SqlxManager;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub pool: Pool<Postgres>
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let settings = AppSettings::new("application.toml");
    let database = SqlxManager::new(&settings.database)
        .await;

    //sqlx::migrate!().run(&database.pool).await
    //    .expect("Failed to register tables");

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_origin([
            settings.web.client.protocol_url().parse()
                .unwrap()
        ])
        .allow_headers([
            header::AUTHORIZATION, header::CONTENT_TYPE,
        ])
        .allow_methods([
            Method::GET, Method::PUT,
            Method::DELETE, Method::PATCH,
        ]);

    let router = Router::new()
        .nest("/api", api::router())
        .layer(Extension(
            AppState {
                settings: settings.clone(),
                pool: database.pool
            })
        )
        .layer(cors);

    log::info!("Starting axum server with tokio...");
    let listener = tokio::net::TcpListener::bind(settings.web.rest.url())
        .await
        .unwrap();
    axum::serve(listener, router)
        .await
        .expect("Failed to start axum server");
}