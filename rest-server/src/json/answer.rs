use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnswerPayload {
    pub answer_idx: i16,
}