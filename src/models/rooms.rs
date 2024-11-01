use std::sync::OnceLock;
use tokio::time::{sleep, Duration};

use axum::http::StatusCode;
use mongodb::bson::doc;
use mongodb::{bson::oid::ObjectId, Collection};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::Display;

use crate::databases::mongo::MongoDb;
use crate::routes::rooms::schemas::AnswerQuestionBody;
use crate::utils::error::AppError;
use crate::utils::score::{calculate_score, check_answer};
use crate::utils::serializer::serialize_option_object_id;
static ROOM_MODEL: OnceLock<RoomModel> = OnceLock::new();

pub struct RoomModel {
    collection: Collection<Room>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub name: String,
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    #[serde(rename = "countPlayers")]
    pub count_players: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    #[serde(rename = "_id", serialize_with = "serialize_option_object_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub timer: u32,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Display, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
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
    pub current_question: Option<u32>,
    pub pin: Option<u32>,
    #[serde(rename = "hostName")]
    pub host_name: String,
    pub status: RoomStatus,
    pub players: Option<Vec<Player>>,
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

    pub async fn create(&self, mut room: Room) -> Result<Room, AppError> {
        room.players = Some(vec![]);
        for question in &mut room.questions {
            question.id = Some(ObjectId::new());
        }
        room.pin = Some(rand::random::<u32>() % 1_000_000);
        room.current_question = Some(1);
        let result = self
            .collection
            .insert_one(&room)
            .await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        room.id = Some(result.inserted_id.as_object_id().unwrap());
        Ok(room)
    }

    pub async fn insert_player(&self, pin: i32, player_name: String) -> Result<Room, AppError> {
        let filter = doc! { "pin": pin };
        let room = self
            .collection
            .find_one(filter.clone())
            .await
            .map_err(|_| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error finding room".to_string(),
                )
            })?;

        if let Some(room) = room {
            if let Some(players) = &room.players {
                if players.iter().any(|player| player.name == player_name) {
                    return Err(AppError::new(
                        StatusCode::BAD_REQUEST,
                        "Player already in the room".to_string(),
                    ));
                }
            };
            let update = doc! {
                "$push": {
                    "players": {
                        "name": player_name,
                        "score": 0
                    }
                }
            };

            self.collection
                .update_one(filter, update)
                .await
                .map_err(|_| {
                    AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error inserting player".to_string(),
                    )
                })?;
            Ok(room)
        } else {
            Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "Room not found".to_string(),
            ))
        }
    }

    pub async fn update_status(&self, room_id: String, status: RoomStatus) -> Result<(), AppError> {
        let room_id = ObjectId::parse_str(&room_id)
            .map_err(|e| AppError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
        let filter = doc! { "_id": room_id };
        let update = doc! { "$set": { "status": status.to_string() } };
        self.collection
            .update_one(filter.clone(), update)
            .await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if status == RoomStatus::Countdown {
            let collection = self.collection.clone();
            tokio::spawn(async move {
                sleep(Duration::from_secs(5)).await;
                let update = doc! { "$set": { "status": RoomStatus::Start.to_string() } };
                if let Err(e) = collection.update_one(filter, update).await {
                    eprintln!("Failed to update room status to start: {}", e);
                }
            });
        }

        Ok(())
    }

    pub async fn update_answer(
        &self,
        room_id: String,
        question_id: String,
        payload: AnswerQuestionBody,
    ) -> Result<(), AppError> {
        dbg!(&payload);
        let room_id = ObjectId::parse_str(&room_id)
            .map_err(|e| AppError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
        let question_id = ObjectId::parse_str(&question_id)
            .map_err(|e| AppError::new(StatusCode::BAD_REQUEST, e.to_string()))?;
        let filter = doc! { "_id": room_id };
        let room_data = self
            .collection
            .find_one(filter.clone())
            .await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        if let Some(data) = room_data {
            match data
                .questions
                .iter()
                .enumerate()
                .find(|(_i, q)| q.id == Some(question_id))
            {
                Some((question_index, question)) => {
                    let question_timer = question.timer * 1000; // convert to ms
                    let mut score: u32 = 0;
                    let mut choice_index: usize = 0;
                    match check_answer(question.choices.clone(), payload.answer) {
                        Ok((c_i, is_correct)) => {
                            dbg!(&c_i, &is_correct);
                            if is_correct {
                                score = calculate_score(question_timer, payload.remain_timer);
                                dbg!(&score);
                                choice_index = c_i;
                            }
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }

                    let (player_index, previous_score): (usize, i32);
                    if let Some(players) = data.players {
                        player_index = players
                            .iter()
                            .enumerate()
                            .find(|(_i, p)| p.name == payload.player_name)
                            .map(|(i, _)| i)
                            .ok_or(AppError::new(
                                StatusCode::BAD_REQUEST,
                                "Player not found".to_string(),
                            ))?;
                        previous_score = players[player_index].score;
                    } else {
                        return Err(AppError::new(
                            StatusCode::BAD_REQUEST,
                            "Players not found".to_string(),
                        ));
                    }

                    let update_score = doc! {
                        "$set": {
                            format!("players.{}.score", player_index): score + previous_score as u32
                        },
                        "$inc": {
                            format!("questions.{}.choices.{}.countPlayers", question_index, choice_index): 1
                        }
                    };

                    self.collection
                        .update_one(filter.clone(), update_score)
                        .await
                        .map_err(|e| {
                            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                        })?;
                    Ok(())
                }
                None => Err(AppError::new(
                    StatusCode::NOT_FOUND,
                    "Question not found".to_string(),
                )),
            }
        } else {
            Err(AppError::new(
                StatusCode::NOT_FOUND,
                "Room not found".to_string(),
            ))
        }
    }

    pub async fn get_by_id(&self, room_id: String) -> Result<Room, AppError> {
        let room_id = ObjectId::parse_str(&room_id)
            .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Invalid room id".to_string()))?;
        let filter = doc! { "_id": room_id };
        let room = self.collection.find_one(filter).await.map_err(|_e| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error finding room".to_string(),
            )
        })?;

        room.ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "Room not found".to_string()))
    }

    pub async fn next_question(&self, room_id: String) -> Result<(), AppError> {
        let room_id = ObjectId::parse_str(&room_id)
            .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Invalid room id".to_string()))?;
        let filter = doc! { "_id": room_id };
        let room = self
            .collection
            .find_one(filter.clone())
            .await
            .map_err(|_e| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error finding room".to_string(),
                )
            })?;

        if let Some(room) = room {
            if room.status != RoomStatus::Summarize {
                return Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "Room status must be 'summarize' to proceed to the next question".to_string(),
                ));
            }
        } else {
            return Err(AppError::new(
                StatusCode::NOT_FOUND,
                "Room not found".to_string(),
            ));
        }

        let update = doc! {
            "$inc": { "currentQuestion": 1 },
            "$set": { "status": RoomStatus::Countdown.to_string() }
        };
        self.collection
            .update_one(filter.clone(), update)
            .await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let collection = self.collection.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(5)).await;
            let update = doc! { "$set": { "status": RoomStatus::Start.to_string() } };
            if let Err(e) = collection.update_one(filter, update).await {
                eprintln!("Failed to update room status to start: {}", e);
            }
        });

        Ok(())
    }

    pub async fn end_question(&self, room_id: String) -> Result<(), AppError> {
        let room_id = ObjectId::parse_str(&room_id)
            .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Invalid room id".to_string()))?;
        let filter = doc! { "_id": room_id };
        let update = doc! {
            "$set": {
                "status": RoomStatus::End.to_string(),
                "currentQuestion": 1,
                "players": []
            }
        };
        self.collection
            .update_one(filter, update)
            .await
            .map_err(|e| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(())
    }
}
