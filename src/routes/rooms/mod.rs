use axum::{
    routing::{post, put},
    Router,
};

mod handlers;
mod schemas;

pub fn room_router() -> Router {
    Router::new()
        .route("/rooms", post(handlers::post_room))
        .route("/rooms/:id/join", post(handlers::join_room))
        .route("/rooms/:id/status", put(handlers::update_room_status))
}
