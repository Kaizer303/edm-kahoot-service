use axum::{
    routing::{get, post},
    Router,
};

mod handlers;
mod schemas;

pub fn room_router() -> Router {
    Router::new()
        .route("/rooms", post(handlers::post_room))
        .route("/rooms/:id/join", post(handlers::join_room))
        .route("/rooms/:id", get(handlers::get_room))
}
