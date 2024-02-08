use axum::Json;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String
}

pub fn json_error(status: http::StatusCode, message: &str) -> (http::StatusCode, Json<ErrorMessage>) {
    (status, Json(ErrorMessage { message: message.to_string() }))
}