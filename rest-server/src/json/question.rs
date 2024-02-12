use garde::Validate;
use serde::{Deserialize, Serialize};
use common::entities::question::DifficultyLevel;

#[derive(Deserialize, Serialize, Validate)]
pub struct QuestionFilter {
    #[garde(length(min = 1, max = 10))]
    pub tags: Option<Vec<i16>>,
    #[garde(range(min = 2015, max = 2024))]
    pub year: Option<i32>,
    #[garde(skip)]
    pub origin: Option<i32>,
    #[garde(skip)]
    pub difficulty: Option<DifficultyLevel>,
    #[serde(flatten)]
    #[garde(dive)]
    pub pagination: Pagination,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct Pagination {
    #[garde(range(min=1, max=50))]
    pub page: i64,
    #[garde(range(min=5, max=50))]
    pub per_page: i64,
}