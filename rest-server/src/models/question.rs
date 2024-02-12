use serde::Serialize;

#[derive(Serialize)]
pub struct Question {
    pub content: String,
    pub options: Vec<String>,
    pub tag: i16,
    pub year: i16,
    pub origin: i16,
    pub difficulty_rating: f64
}