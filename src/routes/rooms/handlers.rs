use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::rooms::{Room, RoomModel},
    utils::error::AppError,
};

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
) -> Result<impl IntoResponse, AppError> {
    let name = payload.name;
    RoomModel::get_instance()
        .insert_player(id, name)
        .await
        .map_err(|e| e)?;

    Ok(Json(format!("Joining room success",)))
}

pub async fn get_room(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let room_model = RoomModel::get_instance();
    let room = room_model.get_by_id(id).await.map_err(|e| e)?;

    Ok(Json(room))
}
