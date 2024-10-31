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
    Path(pin): Path<String>,
    Json(payload): Json<JoinRequest>,
) -> Result<impl IntoResponse, AppError> {
    let name = payload.name;
    let pin: i32 = pin
        .parse::<i32>()
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Invalid pin".to_string()))?;
    RoomModel::get_instance()
        .insert_player(pin, name)
        .await
        .map_err(|e| e)?;

    Ok(Json(format!("Joining room success",)))
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

pub async fn get_room(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    let room_model = RoomModel::get_instance();
    println!("id: {}", id);
    let room = room_model.get_by_id(id).await.map_err(|e| e)?;

    Ok(Json(room))
}

pub async fn next_question(Path(id): Path<String>) -> Result<impl IntoResponse, AppError> {
    RoomModel::get_instance().next_question(id).await?;
    Ok((StatusCode::OK, "Next question started"))
}
