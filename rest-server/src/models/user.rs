use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProtectedUser {
    pub id: i32,
}

#[derive(Serialize)]
pub struct PublicUser {
    pub id: i32,
    pub username: String,
}