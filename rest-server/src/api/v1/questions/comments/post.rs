use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::types::Uuid;
use crate::AppState;
use crate::json::comment::CreateCommentPayload;
use crate::json::error::json_error;

pub async fn create_comment(
    Extension(state): Extension<AppState>,
    Path(question_id): Path<i32>,
    Json(payload): Json<CreateCommentPayload>
) -> impl IntoResponse {
    let query = sqlx::query(
        r#"
            INSERT INTO comments (user_id, question_id, content)
            VALUES ($1, $2, $3)
        "#
    )
        // TODO: get authorized user
        .bind(Uuid::parse_str("fb8e08de-6d66-445a-ab2b-f3f40aabfa2e").unwrap())
        .bind(question_id)
        .bind(payload.content)
        .execute(&state.pool)
        .await;

    if let Err(_) = query {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao salvar comentário")
            .into_response()
    } else {
        StatusCode::OK.into_response()
    }
}