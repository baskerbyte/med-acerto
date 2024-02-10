use garde::Validate;
use serde::{Deserialize, Serialize};
use crate::models::answer::AnswerStats;

#[derive(Deserialize, Validate)]
pub struct AnswerPayload {
    #[garde(range(min = 0, max = 3))]
    pub answer_idx: i16,
}

#[derive(Serialize)]
pub struct AnswerResponse {
    pub correct: bool,
    pub stats: AnswerStats
}