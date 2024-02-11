use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_garde::WithValidation;
use http::StatusCode;
use crate::json::comment::CreateCommentPayload;
use crate::json::error::json_error;
use crate::web::AppState;

pub async fn create_comment(
    Extension(state): Extension<AppState>,
    Path(question_id): Path<i32>,
    WithValidation(payload): WithValidation<Json<CreateCommentPayload>>
) -> impl IntoResponse {
    let query = sqlx::query(
        r#"
            INSERT INTO comments (user_id, question_id, content)
            VALUES ($1, $2, $3)
        "#
    )
        // TODO: get authorized user
        .bind(1)
        .bind(question_id)
        .bind(&payload.content)
        .execute(&state.pool)
        .await;

    if let Err(_) = query {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao salvar comentário")
            .into_response()
    } else {
        StatusCode::OK.into_response()
    }
}