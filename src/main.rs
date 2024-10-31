use axum::extract::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{routing::get, routing::post, Router};
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct JoinRequest {
    name: String,
}

async fn insert_player(room_id: i64, player_name: String) -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("kahoot");
    let collection = database.collection("rooms");

    let filter = doc! { "_id": room_id, "players.name": { "$ne": player_name } };
    let update = doc! {
        "$push": {
            "players": {
                "name": player_name,
                "score": 0
            }
        }
    };

    collection.update_one(filter, update, None).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));
    let app = app.route(
        "/rooms/:id/join",
        post(
            |Path(id): Path<String>, Json(payload): Json<JoinRequest>| async move {
                let name = payload.name;
                let room_id: i64 = id.parse().unwrap();
                match insert_player(room_id, name).await {
                    Ok(_) => (StatusCode::OK, "ok".to_string()),
                    Err(_) => (StatusCode::BAD_REQUEST, "can't join the room".to_string()),
                }
            },
        ),
    );
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
