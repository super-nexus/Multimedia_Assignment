use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Baloon {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub lat: f32,
    pub lng: f32,
    pub message: String,
    pub owner: String,
    pub timestamp: i64,
    pub popped: bool,
    pub popped_at: i64
}