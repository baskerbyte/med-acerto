use axum::{Extension, Json};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum_garde::WithValidation;
use http::StatusCode;
use sqlx::types::Uuid;
use crate::AppState;
use crate::json::error::json_error;
use crate::json::question::Pagination;
use crate::models::comment::Comment;

pub async fn question_comments(
    Extension(state): Extension<AppState>,
    Path(question_id): Path<i32>,
    WithValidation(pagination): WithValidation<Query<Pagination>>,
) -> impl IntoResponse {
    let offset = (pagination.page -  1) * pagination.per_page;

    let comments = sqlx::query_as::<_, Comment>(
        r#"
        SELECT
            c.user_id, c.content,
            u.username AS user_username, u.avatar AS user_avatar,
            COUNT(l.id) AS likes,
            EXISTS(SELECT 1 FROM comment_likes l WHERE l.comment_id = c.id AND l.user_id = $1) AS liked
        FROM comments c
        INNER JOIN users u ON u.id = c.user_id
        LEFT JOIN comment_likes l ON l.comment_id = c.id
        WHERE c.question_id = $2
        GROUP BY c.id, u.username, u.avatar
        ORDER BY COUNT(l.id) DESC
        LIMIT $3 OFFSET $4;
        "#
    )
        // TODO: get authorized user
        .bind(Uuid::parse_str("fb8e08de-6d66-445a-ab2b-f3f40aabfa2e").unwrap())
        .bind(question_id)
        .bind(pagination.per_page)
        .bind(offset)
        .fetch_all(&state.pool)
        .await;

    if let Ok(comments) = comments {
        Json(comments).into_response()
    } else {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao buscar comentários")
            .into_response()
    }
}