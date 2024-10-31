mod databases;
mod models;
mod routes;
mod utils;

use databases::mongo::MongoDb;
use routes::router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    MongoDb::init().await;

    let routes = router();

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
