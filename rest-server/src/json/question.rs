use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
}