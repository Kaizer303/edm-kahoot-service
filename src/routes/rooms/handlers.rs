use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::models::rooms::{Room, RoomModel};

use super::schemas::JoinRequest;

pub async fn post_room(Json(room): Json<Room>) -> Result<impl IntoResponse, impl IntoResponse> {
    let room_model = RoomModel::get_instance();
    let result = room_model
        .create(room)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok::<(StatusCode, Json<Room>), (StatusCode, String)>((StatusCode::CREATED, Json(result)))
}

pub async fn join_room(
    Path(id): Path<String>,
    Json(payload): Json<JoinRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let name = payload.name;
    match RoomModel::get_instance().insert_player(id, name).await {
        Ok(_) => Ok((StatusCode::OK, "ok".to_string())),
        Err(_) => Err((StatusCode::BAD_REQUEST, "can't join the room".to_string())),
    }
}
