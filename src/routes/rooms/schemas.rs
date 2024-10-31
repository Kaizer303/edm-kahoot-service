use serde::Deserialize;

#[derive(Deserialize)]
pub struct JoinRequest {
    pub name: String,
}
