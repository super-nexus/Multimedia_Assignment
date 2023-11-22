mod weather;
mod baloon;
mod persistance;

use weather::model::ApiResponse;
use baloon::model::{Baloon, Latlng};
use std::collections::HashMap;
use dotenv::dotenv;
use rand::Rng;
use mongodb::Client;

const MAX_DISTANCE_KM: f32 = 10.0;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let mut weather_data: Vec<ApiResponse> = Vec::new();
    let mongo_client = persistance::mongo::get_client().await;

    loop {
        println!("Running loop");
        println!("Remove outdated weather data");
        weather::util::remove_outdated_weather(&mut weather_data);

        println!("Fetch baloons");
        let (mut baloons, popped_baloons) = persistance::mongo::get_baloons_and_popped_baloons(&mongo_client).await;        

        println!("Clean popped baloons");
        clean_popped_baloons(&mongo_client, &popped_baloons).await;

        println!("Updating popped baloons");
        update_popped_baloons(&mongo_client, &mut baloons).await;

        println!("Clustering baloons");
        let mut clustered_baloons: HashMap<String, Vec<Baloon>> = baloon::util::cluster_baloons(baloons);

        println!("Updating baloons");
        update_baloons(&mut clustered_baloons, &mut weather_data).await;

        println!("Store updated baloons");
        store_baloons(&mongo_client, &clustered_baloons).await;

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
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

async fn store_baloons(client: &Client, baloons_cluster: &HashMap<String, Vec<Baloon>>) {
    // Map all baloons to a single vector
    let all_baloons: Vec<Baloon> = baloons_cluster.values()
        .flat_map(|baloons| baloons.iter().cloned())
        .collect();
    
    persistance::mongo::update_baloons(client, &all_baloons).await;
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
                .await
                .ok()
                .expect("Could not fetch weather data");
            
            weather_data.push(weather.clone());
            weather
        }
    }    
}

// Remove baloons that are older than 30 mins
async fn clean_popped_baloons(client: &Client, popped_baloons: &Vec<Baloon>) {
    let current_time = chrono::offset::Utc::now().timestamp();
    let outdated_baloons: Vec<Baloon> = popped_baloons.iter().filter(|baloon| {
        let baloon_time_in_air_mins = (current_time - baloon.popped_at) / 60;
        baloon_time_in_air_mins > 30
    }).cloned().collect();

    persistance::mongo::delete_baloons(client, &outdated_baloons).await;
}

async fn update_popped_baloons(client: &Client, baloons: &mut Vec<Baloon>) {
    let current_time = chrono::offset::Utc::now().timestamp();
    let max_baloon_time_in_air_mins = 15;

    let mut new_popped_baloons: Vec<Baloon> = baloons.iter().filter(|baloon| {
        let baloon_time_in_air_mins = (current_time - baloon.timestamp) / 60;
        let rand_number = rand::thread_rng().gen_range(0..max_baloon_time_in_air_mins);

        baloon_time_in_air_mins > rand_number
    }).cloned().collect();

    for baloon in &mut new_popped_baloons {
        println!("Baloon popped at {}, {}", baloon.lat, baloon.lng);
        baloon.popped = true;
        baloon.popped_at = current_time;
    }

    persistance::mongo::update_baloons(client, &new_popped_baloons).await;

    // Delete the popped baloons from baloons
    baloons.retain(|baloon| !new_popped_baloons.contains(baloon));
}