use axum::{routing::post, Router};

mod handlers;
mod schemas;

pub fn room_router() -> Router {
    Router::new()
        .route("/rooms", post(handlers::post_room))
        .route("/rooms/:id/join", post(handlers::join_room))
}
