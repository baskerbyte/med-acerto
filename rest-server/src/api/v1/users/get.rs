use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use crate::json::error::json_error;
use crate::models::user::PublicUser;
use crate::web::AppState;

pub async fn user_info(
    Extension(state): Extension<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoResponse {
    match sqlx::query_as!(
        PublicUser,
        "SELECT id, username FROM users WHERE id = $1",
        user_id
    )
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
}