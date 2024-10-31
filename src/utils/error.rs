use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppError {
    pub error: String,
}
