use serde::Deserialize;
use common::entities::question::DifficultyLevel;

#[derive(Deserialize)]
pub struct QuestionFilter {
    pub tags: Option<Vec<i16>>,
    pub year: Option<i16>,
    pub origin: Option<i16>,
    pub difficulty: Option<DifficultyLevel>,
    pub page: i16,
    pub per_page: i16,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: i16,
    pub per_page: i16,
}