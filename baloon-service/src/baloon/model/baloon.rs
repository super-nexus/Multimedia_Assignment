use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Baloon {
    pub lat: f32,
    pub lng: f32,
    pub message: String,
    pub owner: String
}