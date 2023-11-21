use chrono::{Duration, Utc, DateTime, Timelike};
use crate::weather::model::{ApiResponse, Weather};
use std::env;

pub async fn fetch_weather_data(lat: f32, lng: f32) -> Result<ApiResponse, reqwest::Error> {
    let api_key = env::var("OPEN_WEATHER_MAP_API_KEY").expect("OPEN_WEATHER_MAP_API_KEY not set");
    let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}", lat, lng, api_key);

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    let json: ApiResponse = serde_json::from_str(&body).expect("Could not parse json");

    Ok(json)
}

pub fn remove_outdated_weather(weather_data: &mut Vec<ApiResponse>) {
    weather_data.retain(|weather| !data_outdated(weather));
}

pub fn get_weather_data_for_current_hour(weather: &ApiResponse) -> Weather {
    let now: DateTime<Utc> = Utc::now();
    let current_hour: u32 = now.hour();
    let mut closest_weather: Option<Weather> = None;
    let mut min_distance = 24;

    for weather in &weather.hourly {
        let weather_hour: u32 = DateTime::<Utc>::from_timestamp(weather.dt, 0).expect("Could not parse timestamp").hour();
        let distance = if weather_hour > current_hour {weather_hour - current_hour} else {current_hour - weather_hour};

        if distance < min_distance {
            min_distance = distance;
            closest_weather = Some(weather.clone());
        }
    }

    closest_weather.expect("Could not find weather data for current hour")
}

fn data_outdated(weather: &ApiResponse) -> bool {
    let now: DateTime<Utc> = Utc::now();
    let last_update: DateTime<Utc> = DateTime::<Utc>::from_timestamp(weather.current.dt, 0).expect("Could not parse timestamp");
    let duration: Duration = now - last_update;

    duration > Duration::hours(45)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test as tokio_test;
    use dotenv::dotenv;

    #[tokio_test]
    async fn test_fetch_weather_data() {
        dotenv().ok();

        let lat = 51.50;
        let lng = 4.49;
        let weather_data = fetch_weather_data(lat, lng).await;

        assert!(weather_data.is_ok());
    }
}