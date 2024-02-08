use serde::Deserialize;

#[derive(Deserialize)]
pub struct QuestionFilter {
    pub tags: Option<Vec<i16>>,
    pub year: Option<i16>,
    pub origin: Option<i16>,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
}