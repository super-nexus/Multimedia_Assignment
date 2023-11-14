use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub lat: f32,
    pub lon: f32,
    pub current: Weather,
    pub hourly: Vec<Weather>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub dt: i64,
    pub wind_speed: f32,
    pub wind_deg: u16,
    pub wind_gust: f32,
}