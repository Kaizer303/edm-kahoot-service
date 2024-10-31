use std::sync::OnceLock;

use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::{bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::databases::mongo::MongoDb;
use crate::utils::serializer::serialize_option_object_id;
static ROOM_MODEL: OnceLock<RoomModel> = OnceLock::new();

pub struct RoomModel {
    collection: Collection<Room>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub name: String,
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    #[serde(rename = "countPlayers")]
    pub count_players: i32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    #[serde(rename = "_id", serialize_with = "serialize_option_object_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub timer: i32,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoomStatus {
    Wait,
    Countdown,
    Start,
    Summarize,
    End,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    #[serde(rename = "_id", serialize_with = "serialize_option_object_id")]
    pub id: Option<ObjectId>,
    #[serde(rename = "currentQuestion")]
    pub current_question: i32,
    pub pin: i32,
    #[serde(rename = "hostID", serialize_with = "serialize_option_object_id")]
    pub host_id: Option<ObjectId>,
    pub status: RoomStatus,
    pub players: Vec<Player>,
    pub questions: Vec<Question>,
}

impl RoomModel {
    pub fn initialize() {
        let collection = MongoDb::get_instance().db.collection::<Room>("rooms");
        ROOM_MODEL.get_or_init(|| RoomModel { collection });
    }

    pub fn get_instance() -> &'static RoomModel {
        ROOM_MODEL.get().unwrap()
    }

    pub async fn create(&self, mut room: Room) -> Result<Room, mongodb::error::Error> {
        room.host_id = Some(ObjectId::new());
        for question in &mut room.questions {
            question.id = Some(ObjectId::new());
        }
        let result = self.collection.insert_one(&room).await?;
        room.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(room)
    }

    pub async fn insert_player(
        &self,
        room_id: String,
        player_name: String,
    ) -> mongodb::error::Result<()> {
        let room_id = ObjectId::parse_str(&room_id).unwrap();
        let filter = doc! { "_id": room_id, "players.name": { "$ne": &player_name } };
        let update = doc! {
            "$push": {
                "players": {
                    "name": player_name,
                    "score": 0
                }
            }
        };

        self.collection.update_one(filter, update).await?;
        Ok(())
    }
}
