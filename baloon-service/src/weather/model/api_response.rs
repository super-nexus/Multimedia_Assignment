use serde::{Deserialize, Serialize};
use crate::weather::model::Weather;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    pub lat: f32,
    pub lon: f32,
    pub current: Weather,
    pub hourly: Vec<Weather>
}