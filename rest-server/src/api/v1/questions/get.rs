use axum::{Extension, Json};
use axum::extract::Query;
use axum::response::IntoResponse;
use http::StatusCode;
use crate::AppState;
use crate::json::error::json_error;
use crate::json::question::QuestionFilter;
use crate::models::question::Question;

pub async fn filter_questions(
    Extension(state): Extension<AppState>,
    Query(filter): Query<QuestionFilter>,
) -> impl IntoResponse {
    let offset = (filter.page -  1) * filter.per_page;

    let questions = sqlx::query_as::<_, Question>(
        r#"
            SELECT
                content, options, tag,
                year, origin
            FROM questions
            WHERE ($1::SMALLINT[] IS NULL OR tag = ANY($1::SMALLINT[]))
              AND ($2::INTEGER IS NULL OR year = $2)
              AND ($3::VARCHAR IS NULL OR origin = $3)
            ORDER BY RANDOM()
            LIMIT $4 OFFSET $5
        "#)
        .bind(filter.tags)
        .bind(filter.year)
        .bind(filter.origin)
        .bind(filter.per_page)
        .bind(offset)
        .fetch_all(&state.pool)
        .await;

    if let Ok(questions) = questions {
        Json(questions).into_response()
    } else {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao buscar questões")
            .into_response()
    }
}