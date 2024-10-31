use std::sync::OnceLock;

use mongodb::{Client, Database};

use crate::models::rooms::RoomModel;

static MONGO_DB: OnceLock<MongoDb> = OnceLock::new();

pub struct MongoDb {
    // pub client: Client,
    pub db: Database,
}

impl MongoDb {
    pub async fn init() {
        println!("Connecting to MongoDB");
        let client = Client::with_uri_str(
            "mongodb://mongo:ObTlLrUPKILhkAqQLBGOUWXxbhDZTmvO@autorack.proxy.rlwy.net:59927",
        )
        .await
        .expect("Can't connect to MongoDB");

        let db = client.database("kahoot");

        println!("Connected to MongoDB");
        MONGO_DB.get_or_init(|| MongoDb { db });

        RoomModel::initialize();
    }

    pub fn get_instance() -> &'static MongoDb {
        MONGO_DB.get().unwrap()
    }
}
