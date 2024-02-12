use serde::Serialize;

#[derive(Serialize)]
pub struct AnswerStatistic {
    pub answer_idx: i16,
    pub number_of_users: Option<i64>,
}

#[derive(Serialize)]
pub struct AnswerStats {
    pub total_answers: Option<i64>,
    pub total_correct_answers: Option<i64>
}