use axum::{Extension, Json};
use axum::extract::Query;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::types::Uuid;
use crate::AppState;
use crate::json::error::json_error;
use crate::json::question::QuestionFilter;
use crate::models::question::Question;

pub async fn filter_questions(
    Extension(state): Extension<AppState>,
    Query(filter): Query<QuestionFilter>,
) -> impl IntoResponse {
    let offset = (filter.page -  1) * filter.per_page;

    // TODO: Add support to filter by difficulty
    let questions = sqlx::query_as::<_, Question>(r#"
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
        "#)
        .bind(filter.tags)
        .bind(filter.year)
        .bind(filter.origin)
        .bind(filter.per_page)
        .bind(offset)
        // TODO: get authorized user
        .bind(Uuid::parse_str("fb8e08de-6d66-445a-ab2b-f3f40aabfa2e").unwrap())
        .fetch_all(&state.pool)
        .await;

    if let Ok(questions) = questions {
        Json(questions).into_response()
    } else {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao buscar questões")
            .into_response()
    }
}