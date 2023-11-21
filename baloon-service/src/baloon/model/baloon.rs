use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Baloon {
    pub id: String,
    pub lat: f32,
    pub lng: f32,
    pub message: String,
    pub owner: String,
    pub timestamp: i64
}