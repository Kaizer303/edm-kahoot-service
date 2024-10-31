mod databases;
mod models;
mod routes;
mod utils;

use databases::mongo::MongoDb;
use routes::router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    MongoDb::init().await;

    let cors = CorsLayer::new().allow_origin(Any);
    let routes = router().layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
