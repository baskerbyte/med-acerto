use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_garde::WithValidation;
use http::StatusCode;
use crate::json::comment::UpdateCommentPayload;
use crate::json::error::json_error;
use crate::web::AppState;

pub async fn update_comment(
    Extension(state): Extension<AppState>,
    Path(comment_id): Path<i32>,
    WithValidation(payload): WithValidation<Json<UpdateCommentPayload>>
) -> impl IntoResponse {
    let author = match sqlx::query_scalar!(
        r#"
            SELECT user_id FROM comments
            WHERE id = $1;
        "#,
        comment_id
    )
        .fetch_one(&state.pool)
        .await {
        Ok(result) => result,
        Err(_) => {
            return json_error(StatusCode::NOT_FOUND, "Falha ao encontrar author")
                .into_response();
        }
    };

    // TODO: get authorized user
    if author != 1 {
        return json_error(
            StatusCode::UNAUTHORIZED,
            "Somente o autor pode editar o comentário"
        )
            .into_response();
    }

    if let Err(_) = sqlx::query!(
        r#"
            UPDATE comments
            SET content = $1, was_edited = TRUE
            WHERE id = $2;
        "#,
        &payload.content,
        comment_id
    )
        .execute(&state.pool)
        .await {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao atualizar comentário")
            .into_response()
    } else {
        StatusCode::OK.into_response()
    }
}