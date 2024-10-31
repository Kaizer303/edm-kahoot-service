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
