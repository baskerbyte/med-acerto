use std::env::var;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct WebApp {
    pub rest: RestServer,
    pub client: WebClient,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RestServer {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WebClient {
    pub host: String,
    pub port: u16,
}

impl Default for RestServer {
    fn default() -> Self {
        Self {
            host: var("REST_SERVER_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("REST_SERVER_PORT")
                .unwrap_or("5443".to_string())
                .parse().unwrap(),
        }
    }
}

impl Default for WebClient {
    fn default() -> Self {
        Self {
            host: var("WEB_CLIENT_HOST")
                .unwrap_or("localhost".to_string()),
            port: var("WEB_CLIENT_PORT")
                .unwrap_or("5173".to_string())
                .parse().unwrap(),
        }
    }
}