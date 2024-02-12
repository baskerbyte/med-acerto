use axum::Extension;
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use crate::json::error::json_error;
use crate::web::AppState;

pub async fn delete_comment(
    Extension(state): Extension<AppState>,
    Path(comment_id): Path<i32>,
) -> impl IntoResponse {
    let author: i32 = match sqlx::query_scalar!(
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
            "Somente o autor ou moderador podem apagar o comentário"
        )
            .into_response();
    }

    if let Err(_) = sqlx::query(
        r#"
            DELETE FROM comments
            WHERE id = $
        "#
    )
        .bind(comment_id)
        .execute(&state.pool)
        .await {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao deletar comentário")
            .into_response()
    } else {
        StatusCode::OK.into_response()
    }
}