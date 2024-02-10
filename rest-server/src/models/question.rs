use serde::Serialize;
use sqlx::{Error, Row};
use sqlx::postgres::PgRow;
use common::entities::question::DifficultyLevel;

#[derive(Serialize)]
pub struct Question {
    content: String,
    options: Vec<String>,
    tag: i16,
    year: i16,
    origin: i16,
    difficulty_rating: DifficultyLevel
}

impl<'r> sqlx::FromRow<'r, PgRow> for Question {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let difficulty_rating = match row.get::<f64, _>("difficulty_rating") {
            rating if rating <= 1.2 => DifficultyLevel::VeryEasy,
            rating if rating <= 1.4 => DifficultyLevel::Easy,
            rating if rating <= 1.6 => DifficultyLevel::Medium,
            rating if rating <= 1.8 => DifficultyLevel::Hard,
            _ => DifficultyLevel::VeryHard,
        };

        Ok(
            Self {
                content: row.get("content"),
                options: row.get("options"),
                tag: row.get("tag"),
                year: row.get("year"),
                origin: row.get("origin"),
                difficulty_rating,
            }
        )
    }
}