use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::types::Uuid;
use crate::AppState;
use crate::json::error::json_error;
use crate::models::user::PublicUser;

pub async fn user_info(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    if let Ok(id) = Uuid::parse_str(&user_id) {
        match sqlx::query_as::<_, PublicUser>("SELECT username FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.pool)
            .await
        {
            Ok(Some(user)) => Json(user).into_response(),
            Ok(None) => json_error(StatusCode::NOT_FOUND, "Usuário não encontrado")
                .into_response(),
            Err(_) => json_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Falha ao encontrar usuário"
            )
                .into_response()
        }
    } else {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao reconhecer ID")
            .into_response()
    }
}