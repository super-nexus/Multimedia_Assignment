mod weather;
mod baloon;

use weather::model::ApiResponse;
use baloon::model::{Baloon, Latlng};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use dotenv::dotenv;

const MAX_DISTANCE_KM: f32 = 10.0;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let mut weather: Option<ApiResponse> = None;

    let file_path = "/Users/andrijakuzmanov/Documents/code/faks/MULTI/Multimedia_Assignment/baloons.json";
    let mut weather_data: Vec<ApiResponse> = Vec::new();

    loop {
        println!("1. Running loop");
        println!("2. Remove outdated weather data");
        weather::util::remove_outdated_weather(&mut weather_data);

        println!("3. Fetching baloons");
        let baloons: Vec<Baloon> = get_baloons_data(file_path).unwrap_or_else(|| Vec::new());

        println!("4. Clustering baloons");
        let mut clustered_baloons: HashMap<String, Vec<Baloon>> = baloon::util::cluster_baloons(baloons);

        println!("5. Updating baloons");
        update_baloons(&mut clustered_baloons, &mut weather_data).await;

        // Pop baloons here

        println!("6. Store updated baloons");
        store_baloons(&clustered_baloons, file_path).await;

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

fn get_baloons_data(path: &str) -> Option<Vec<Baloon>> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).ok()?;
    serde_json::from_str(&contents).ok()?
}

async fn update_baloons(baloons_cluster: &mut HashMap<String, Vec<Baloon>>, weather_data: &mut Vec<ApiResponse>) {
    for (key, baloons) in baloons_cluster.iter_mut() {
        let closest_weather_data = get_closest_weather_data(key, weather_data).await;
        let weather_for_current_hour = weather::util::get_weather_data_for_current_hour(&closest_weather_data);
        
        // Update baloons
        for baloon in baloons {
            // update baloon coordinates
            baloon::util::move_baloon(baloon, &weather_for_current_hour);
        }
    }
}

async fn store_baloons(baloons_cluster: &HashMap<String, Vec<Baloon>>, file_path: &str) {
    // Map all baloons to a single vector
    let all_baloons: Vec<Baloon> = baloons_cluster.values()
        .flat_map(|baloons| baloons.iter().cloned())
        .collect();
    
    let json = serde_json::to_string(&all_baloons).expect("Could not serialize baloons");
    let mut file = File::create(file_path).expect("Could not create file");
    file.write_all(json.as_bytes()).expect("Could not write to file");
}

async fn get_closest_weather_data(latlng_key: &str, weather_data: &mut Vec<ApiResponse>) -> ApiResponse {
    let baloon_latlng = Latlng::from_string(latlng_key);
    let mut current_closest_weather: Option<ApiResponse> = None;
    
    for weather in &mut *weather_data {
        let weather_latlng = Latlng { lat: weather.lat, lng: weather.lon };
        let distance = baloon_latlng.distance(&weather_latlng);

        if distance < MAX_DISTANCE_KM {
            current_closest_weather = Some(weather.clone());
        }
    }

    match current_closest_weather {
        Some(weather) => {
            println!("Found weather data for baloon cluster at {}, {}", baloon_latlng.lat, baloon_latlng.lng);
            weather
        },

        None => {
            println!("Fetching weather data for baloon cluster at {}, {}", baloon_latlng.lat, baloon_latlng.lng);
            let weather = weather::util::fetch_weather_data(baloon_latlng.lat, baloon_latlng.lng)
                .await.ok().expect("Could not fetch weather data");
            
            weather_data.push(weather.clone());
            weather
        }
    }    
}
