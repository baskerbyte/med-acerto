use std::sync::Arc;
use axum::{Extension, Router};
use http::{header, Method};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use app_core::database::SqlxManager;
use common::settings::AppSettings;
use crate::api;

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub pool: Pool<Postgres>
}

pub struct App {
    state: AppState,
}

impl App {
    pub async fn new(settings: &str) -> Self {
        env_logger::init();

        let settings = AppSettings::new(settings);
        let database = SqlxManager::new(&settings.database)
            .await;

        sqlx::migrate!().run(&database.pool).await
            .expect("Failed to migrate tables");

        Self {
            state: AppState {
                settings,
                pool: database.pool
            }
        }
    }

    fn router(self) -> Router {
        let cors = CorsLayer::new()
            .allow_credentials(true)
            .allow_origin([
                self.state.settings.web.client.protocol_url().parse()
                    .unwrap()
            ])
            .allow_headers([
                header::AUTHORIZATION, header::CONTENT_TYPE,
            ])
            .allow_methods([
                Method::GET, Method::PUT,
                Method::DELETE, Method::PATCH,
            ]);

        Router::new()
            .nest("/api", api::router())
            .layer(Extension(self.state))
            .layer(cors)
    }

    pub async fn run(self) {
        log::info!("Starting axum server with tokio...");
        let listener = tokio::net::TcpListener::bind(self.state.settings.web.rest.url())
            .await
            .unwrap();
        axum::serve(listener, self.router())
            .await
            .expect("Failed to start axum server");
    }
}