use axum::{Extension, Json};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum_garde::WithValidation;
use http::StatusCode;
use crate::json::error::json_error;
use crate::json::question::QuestionFilter;
use crate::models::question::Question;
use crate::web::AppState;

pub async fn filter_questions(
    Extension(state): Extension<AppState>,
    WithValidation(filter): WithValidation<Query<QuestionFilter>>,
) -> impl IntoResponse {
    let offset = (filter.pagination.page -  1) * filter.pagination.per_page;

    // TODO: Add support to filter by difficulty
    let questions = sqlx::query_as!(
        Question,
        r#"
            SELECT
                q.content, q.options, q.tag,
                q.year, q.origin, q.difficulty_rating
            FROM questions q
            LEFT JOIN answers a ON q.id = a.question_id AND a.user_id != $6
            WHERE ($1::SMALLINT[] IS NULL OR q.tag = ANY($1::SMALLINT[]))
              AND ($2::INTEGER IS NULL OR q.year = $2)
              AND ($3::INTEGER IS NULL OR q.origin = $3)
            ORDER BY RANDOM()
            LIMIT $4 OFFSET $5
        "#,
        filter.tags.as_ref().map(|vec| vec.as_slice()).unwrap_or(&[]),
        filter.year,
        filter.origin,
        filter.pagination.per_page,
        offset,
        // TODO: get authorized user
        1
    )
        .fetch_all(&state.pool)
        .await;

    if let Ok(questions) = questions {
        Json(questions).into_response()
    } else {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao buscar questões")
            .into_response()
    }
}