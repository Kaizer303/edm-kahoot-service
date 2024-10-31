use axum::http::StatusCode;

use crate::models::rooms::Choice;

use super::error::AppError;

const MAX_SCORE: u32 = 1000;

pub fn calculate_score(total_time: u32, remaining_time: u32) -> u32 {
    let score: u32 = (MAX_SCORE as f64 / total_time as f64 * remaining_time as f64) as u32;
    score
}

pub fn check_answer(choices: Vec<Choice>, answer: String) -> Result<(usize, bool), AppError> {
    match choices.iter().enumerate().find(|(_i, c)| c.name == answer) {
        Some((choice_index, choice)) => Ok((choice_index, choice.is_correct)),
        None => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Invalid answer".to_string(),
        )),
    }
}
