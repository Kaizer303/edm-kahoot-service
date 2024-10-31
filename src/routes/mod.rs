use axum::Router;
use rooms::room_router;

pub mod rooms;

pub fn router() -> Router {
    Router::new().merge(room_router())
}
