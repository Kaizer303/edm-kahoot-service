use serde::Deserialize;

use crate::models::rooms::RoomStatus;

#[derive(Deserialize)]
pub struct JoinRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateStatusBody {
    pub status: RoomStatus,
}

#[derive(Deserialize)]
pub struct AnswerQuestionPath {
    pub room_id: String,
    pub question_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnswerQuestionBody {
    pub player_name: String,
    pub remain_timer: u32,
    pub answer: String,
}
