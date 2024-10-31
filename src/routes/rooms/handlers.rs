use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::rooms::{Room, RoomModel},
    utils::error::AppError,
};

use super::schemas::{JoinRequest, UpdateStatusBody};

pub async fn post_room(Json(room): Json<Room>) -> Result<impl IntoResponse, AppError> {
    let room_model = RoomModel::get_instance();
    let result = room_model.create(room).await?;

    Ok((StatusCode::CREATED, Json(result)))
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

pub async fn update_room_status(
    Path(id): Path<String>,
    Json(payload): Json<UpdateStatusBody>,
) -> Result<impl IntoResponse, AppError> {
    RoomModel::get_instance()
        .update_status(id, payload.status)
        .await?;
    Ok((StatusCode::OK, "OK"))
}
