use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use crate::json::error::json_error;
use crate::models::answer::AnswerStatistic;
use crate::web::AppState;

pub async fn question_statics(
    Extension(state): Extension<AppState>,
    Path(question_id): Path<i32>
) -> impl IntoResponse {
    let stats = sqlx::query_as::<_, AnswerStatistic>(r#"
            SELECT answer_idx, COUNT(user_id) AS number_of_users
            FROM answers
            WHERE question_id = $1
            GROUP BY answer_idx
        "#)
        .bind(question_id)
        .fetch_all(&state.pool)
        .await;

    if let Ok(stats) = stats {
        Json(stats).into_response()
    } else {
        json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao buscar estatísticas")
            .into_response()
    }
}