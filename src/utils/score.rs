use axum::http::StatusCode;

use crate::models::rooms::Choice;

use super::error::AppError;

const MAX_SCORE: u32 = 1000;

pub fn calculate_score(total_time: u32, remaining_time: u32) -> u32 {
    let score: u32 = (MAX_SCORE / total_time) * remaining_time;
    score
}

pub fn check_answer(choices: Vec<Choice>, answer: String) -> Result<(String, bool), AppError> {
    match choices.iter().find(|c| c.name == answer) {
        Some(choice) => Ok((choice.name.clone(), choice.is_correct)),
        None => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Invalid answer".to_string(),
        )),
    }
}
