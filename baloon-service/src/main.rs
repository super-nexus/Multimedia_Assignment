mod wind;
mod baloon;

use wind::model::{ApiResponse, Weather};
use baloon::model::{Baloon, Latlng};
use std::collections::HashMap;
use chrono::{Duration, Utc, DateTime, Timelike};
use std::fs::File;
use std::io::prelude::*;

const MAX_DISTANCE_KM: f32 = 10.0;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut weather: Option<ApiResponse> = None;

    const LAT: f32 = 4.48;
    const LNG: f32 = 52.15;

    weather = fetch_weather_data(LAT, LNG).await.ok();
    let file_path = "/Users/andrijakuzmanov/Documents/code/faks/MULTI/Multimedia_Assignment/baloons.json";
    let baloons: Vec<Baloon> = get_baloons_data(file_path).unwrap_or_else(|| Vec::new());
    let mut clustered_baloons: HashMap<String, Vec<Baloon>> = baloon::util::cluster_baloons(baloons);
    let weather_data: Vec<ApiResponse> = Vec::new();
    update_baloons(&mut clustered_baloons, &weather_data).await;

    println!("{:?}", clustered_baloons);

    loop {
        if data_outdated(&weather) {
            weather = fetch_weather_data(LAT, LNG).await.ok();
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

async fn fetch_weather_data(_lat: f32, _lng: f32) -> Result<ApiResponse, reqwest::Error> {
    let url = format!("https://api.openweathermap.org/data/3.0/onecall?lat=4.48&lon=52.15&appid=dbde08b4797828949a4cf02ba7c369fe");
    let response = reqwest::get(&url).await?.json::<ApiResponse>().await?;
    Ok(response)
}

fn data_outdated(weather: &Option<ApiResponse>) -> bool {
    if let Some(current_weather) = weather {
        let now: DateTime<Utc> = Utc::now();
        let last_update: DateTime<Utc> = DateTime::<Utc>::from_timestamp(current_weather.current.dt, 0).expect("Could not parse timestamp");
        let duration: Duration = now - last_update;

        duration > Duration::hours(45)
    } else {
        true
    }
}

fn get_baloons_data(path: &str) -> Option<Vec<Baloon>> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).ok()?;
    serde_json::from_str(&contents).ok()?
}

async fn update_baloons(baloons_cluster: &mut HashMap<String, Vec<Baloon>>, weather_data: &Vec<ApiResponse>) {
    for (key, baloons) in baloons_cluster.iter_mut() {
        let closest_weather_data = get_closest_weather_data(key, weather_data).await;
        let weather_for_current_hour = get_weather_data_for_current_hour(&closest_weather_data);
        
        // Update baloons
        for baloon in baloons {
            // update baloon coordinates
            baloon::util::move_baloon(baloon, &weather_for_current_hour);
        }
    }
}

async fn get_closest_weather_data(latlng_key: &str, weather_data: &Vec<ApiResponse>) -> ApiResponse {
    let baloon_latlng = Latlng::from_string(latlng_key);
    let mut current_closest_weather: Option<ApiResponse> = None;
    
    for weather in weather_data {
        let weather_latlng = Latlng { lat: weather.lat, lng: weather.lon };
        let distance = baloon_latlng.distance(&weather_latlng);

        if distance < MAX_DISTANCE_KM {
            current_closest_weather = Some(weather.clone());
        }
    }

    match current_closest_weather {
        Some(weather) => weather,
        None => 
            fetch_weather_data(baloon_latlng.lat, baloon_latlng.lng)
                .await.ok().expect("Could not fetch weather data")
    }    
}

fn get_weather_data_for_current_hour(weather: &ApiResponse) -> Weather {
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


// For each key in cluster hashmap

// convert to latlng

// get the closest weather data  or if > 10 km away, fetch new data

// calculate the average wind speed and direction

// update the baloons in the cluster with the new data
