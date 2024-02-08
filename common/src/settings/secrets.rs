use std::env::var;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Secrets {
    pub google: OauthSecret,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OauthSecret {
    pub client_id: String,
    pub client_secret: String,
}

impl Default for Secrets {
    fn default() -> Self {
        Self {
            google: OauthSecret {
                client_id: var("GOOGLE_CLIENT_ID")
                    .unwrap(),
                client_secret: var("GOOGLE_CLIENT_SECRET")
                    .unwrap()
            }
        }
    }
}