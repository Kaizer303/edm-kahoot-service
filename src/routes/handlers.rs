use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::rooms::{Room, RoomModel};

pub async fn post_room(Json(room): Json<Room>) -> Result<impl IntoResponse, impl IntoResponse> {
    let room_model = RoomModel::get_instance();
    let result = room_model
        .create(room)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<(StatusCode, Json<Room>), (StatusCode, String)>((StatusCode::CREATED, Json(result)))
}
