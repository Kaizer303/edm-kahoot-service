mod databases;
mod models;
mod routes;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};

use databases::mongo::MongoDb;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    MongoDb::init().await;

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/rooms", post(routes::handlers::post_room));
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
