use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct AnswerStatistic {
    answer_idx: i16,
    number_of_users: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AnswerStats {
    total_answers: i32,
    total_correct_answers: i32
}