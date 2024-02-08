use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::types::Uuid;
use crate::AppState;
use crate::json::error::ErrorMessage;
use crate::models::user::PublicUser;

pub async fn user_info(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if let Ok(id) = Uuid::parse_str(&id) {
        match sqlx::query_as::<_, PublicUser>("SELECT username FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await
        {
            Ok(Some(user)) => {
                Json(user).into_response()
            }
            Ok(None) => {
                (
                    StatusCode::NOT_FOUND,
                    Json(
                        ErrorMessage {
                            message: "Usuário não encontrado".to_string()
                        }
                    )
                ).into_response()
            }
            Err(_) => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        ErrorMessage {
                            message: "Falha ao encontrar usuário".to_string()
                        }
                    )
                ).into_response()
            }
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                ErrorMessage {
                    message: "Falha ao reconhecer ID".to_string()
                }
            )
        ).into_response()
    }
}