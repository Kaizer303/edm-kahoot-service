use axum::{
    routing::{get, post, put},
    Router,
};

mod handlers;
mod schemas;

pub fn room_router() -> Router {
    Router::new()
        .route("/rooms", post(handlers::post_room))
        .route("/rooms/:pin/join", post(handlers::join_room))
        .route("/rooms/:id", get(handlers::get_room))
        .route("/rooms/:id/status", put(handlers::update_room_status))
        .route("/rooms/:id/questions/next", put(handlers::next_question))
        .route("/rooms/:id/questions/end", put(handlers::end_question))
}
