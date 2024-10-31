use crate::models::rooms::Choice;

const MAX_SCORE: u32 = 1000;

pub fn calculate_score(total_time: u32, remaining_time: u32) -> u32 {
    let score: u32 = (MAX_SCORE / total_time) * remaining_time;
    score
}

pub fn check_answer(choices: Vec<Choice>, answer: String) -> bool {
    choices.iter().any(|c| c.name == answer && c.is_correct)
}
