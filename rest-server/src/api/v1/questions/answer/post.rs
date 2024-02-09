use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::types::Uuid;
use crate::AppState;
use crate::json::answer::AnswerPayload;
use crate::json::error::json_error;

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

    if let Err(_) = sqlx::query(r#"
           INSERT INTO answers (user_id, question_id, correct, answer_idx)
           VALUES ($1, $2, $3, $4)
    "#)
        // TODO: get authorized user
        .bind(Uuid::parse_str("fb8e08de-6d66-445a-ab2b-f3f40aabfa2e").unwrap())
        .bind(question_id)
        .bind(payload.answer_idx == row.0)
        .bind(payload.answer_idx)
        .execute(&mut *tx)
        .await {
        return json_error(StatusCode::INTERNAL_SERVER_ERROR, "Erro ao inserir resposta")
            .into_response();
    }

    match tx.commit().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => {
            json_error(StatusCode::INTERNAL_SERVER_ERROR, "Falha ao confirmar transação")
                .into_response()
        },
    }
}