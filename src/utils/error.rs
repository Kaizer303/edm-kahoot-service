use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppError {
    pub status: u16,
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

impl AppError {
    pub fn new(status: StatusCode, error: String) -> Self {
        Self {
            status: status.as_u16(),
            error,
        }
    }
}
