use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Weather {
    pub dt: i64,
    pub wind_speed: f32,
    pub wind_deg: u16,
    pub wind_gust: f32,
}