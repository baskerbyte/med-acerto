use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::types::Uuid;
use crate::AppState;
use crate::json::answer::{AnswerPayload, AnswerResponse};
use crate::json::error::json_error;
use crate::models::answer::AnswerStats;

pub async fn answer_question(
    Extension(state): Extension<AppState>,
    Path(question_id): Path<i32>,
    Json(payload): Json<AnswerPayload>
) -> impl IntoResponse {
    let mut tx = state.pool.begin().await.unwrap();

    let row: (i16,) = match sqlx::query_as(
        r#"
            SELECT answer_idx FROM questions WHERE id = $1
        "#
    )
        .bind(question_id)
        .fetch_one(&mut *tx)
        .await {
        Ok(result) => result,
        Err(_) => {
            return json_error(StatusCode::NOT_FOUND, "Falha ao encontrar questão")
                .into_response();
        }
    };

    let correct = payload.answer_idx == row.0;
    if let Err(_) = sqlx::query(r#"
           INSERT INTO answers (user_id, question_id, correct, answer_idx)
           VALUES ($1, $2, $3, $4)
    "#)
        // TODO: get authorized user
        .bind(Uuid::parse_str("fb8e08de-6d66-445a-ab2b-f3f40aabfa2e").unwrap())
        .bind(question_id)
        .bind(correct)
        .bind(payload.answer_idx)
        .execute(&mut *tx)
        .await {
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "Erro ao inserir resposta")
            .into_response();
    }

    match tx.commit().await {
        Ok(_) => {
            let stats = sqlx::query_as::<_, AnswerStats>(
                r#"
                    SELECT COUNT(*) AS total_users,
                    COUNT(CASE WHEN correct THEN 1 END) AS total_correct_answers
                    FROM answers
                    WHERE question_id = $1;
                "#
            )
                .bind(question_id)
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