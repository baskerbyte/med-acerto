use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum_garde::WithValidation;
use http::StatusCode;
use crate::json::answer::{AnswerPayload, AnswerResponse};
use crate::json::error::json_error;
use crate::models::answer::AnswerStats;
use crate::web::AppState;

pub async fn answer_question(
    Extension(state): Extension<AppState>,
    Path(question_id): Path<i32>,
    WithValidation(payload): WithValidation<Json<AnswerPayload>>
) -> impl IntoResponse {
    let mut tx = state.pool.begin().await.unwrap();

    let row = match sqlx::query_scalar!(
        r#"
            SELECT answer_idx FROM questions WHERE id = $1
        "#,
        question_id
    )
        .fetch_one(&mut *tx)
        .await {
        Ok(result) => result,
        Err(_) => {
            return json_error(StatusCode::NOT_FOUND, "Falha ao encontrar questão")
                .into_response();
        }
    };

    let correct = payload.answer_idx == row;
    if let Err(_) = sqlx::query!(
        r#"
           INSERT INTO answers (user_id, question_id, correct, answer_idx)
           VALUES ($1, $2, $3, $4)
        "#,
        // TODO: get authorized user
        1,
        question_id,
        correct,
        payload.answer_idx
    )
        .execute(&mut *tx)
        .await {
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "Erro ao inserir resposta")
            .into_response();
    }

    match tx.commit().await {
        Ok(_) => {
            let stats = sqlx::query_as!(
                AnswerStats,
                r#"
                    SELECT COUNT(*) AS total_answers,
                    COUNT(CASE WHEN correct THEN 1 END) AS total_correct_answers
                    FROM answers
                    WHERE question_id = $1;
                "#,
                question_id
            )
                .fetch_one(&state.pool)
                .await;

            if let Ok(stats) = stats {
                Json(AnswerResponse { correct, stats }).into_response()
            } else {
                json_error(StatusCode::INTERNAL_SERVER_ERROR, "Erro ao retornar resultado da resposta")
                    .into_response()
            }
        },
        Err(_) => {
            json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao confirmar transação")
                .into_response()
        },
    }
}