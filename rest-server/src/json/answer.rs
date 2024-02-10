use serde::{Deserialize, Serialize};
use crate::models::answer::AnswerStats;

#[derive(Deserialize)]
pub struct AnswerPayload {
    pub answer_idx: i16,
}

#[derive(Serialize)]
pub struct AnswerResponse {
    pub correct: bool,
    pub stats: AnswerStats
}