
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::wind::hourly_weather::Weather;
use rand::Rng;

const EARTH_RADIUS_KM: f32 = 6371.0; // Earth's radius in kilometers
const MAX_DISTANCE_KM: f32 = 10.0; // Define a maximum distance for clustering in kilometers

#[derive(Serialize, Deserialize, Debug)]
pub struct Baloons {
    pub baloons: Vec<Baloon>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Baloon {
    pub lat: f32,
    pub lng: f32,
    pub message: String,
    pub owner: String
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Latlng {
    pub lat: f32,
    pub lng: f32
}

impl Latlng {
    pub fn distance(&self, other: &Latlng) -> f32 {
        let dlat = (other.lat - self.lat).to_radians();
        let dlng = (other.lng - self.lng).to_radians();
        let a = (dlat / 2.0).sin().powi(2) +
                self.lat.to_radians().cos() * other.lat.to_radians().cos() *
                (dlng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        EARTH_RADIUS_KM * c
    }

    pub fn from_string(latlng: &str) -> Latlng {
        let parsed = latlng.split(",")
            .map(|part| part.parse::<f32>().expect("Could not parse latlng"))
            .collect::<Vec<f32>>();

        Latlng { lat: parsed[0], lng: parsed[1] }
    }
}

fn create_latlng_key(lat: f32, lng: f32) -> String {
    format!("{:.5},{:.5}", lat, lng)
}


// Cluster balloons function
pub fn cluster_baloons(baloons: Vec<Baloon>) -> HashMap<String, Vec<Baloon>> {
    let mut clusters: HashMap<String, Vec<Baloon>> = HashMap::new();

    for baloon in baloons {
        let mut closest_cluster_key: Option<String> = None;
        let mut min_distance = MAX_DISTANCE_KM;
        let baloon_latlng = Latlng { lat: baloon.lat, lng: baloon.lng };

        for (key, _) in clusters.iter() {
            let centroid: Latlng = Latlng::from_string(key);
            let distance = centroid.distance(&baloon_latlng);
            
            if distance < min_distance {
                min_distance = distance;
                closest_cluster_key = Some(key.clone());
            }
        }

        match closest_cluster_key {
            Some(key) => clusters.get_mut(&key).unwrap().push(baloon),
            None => {
                let key = create_latlng_key(baloon.lat, baloon.lng);
                clusters.entry(key).or_insert_with(Vec::new).push(baloon);
            },
        }
    }

    clusters
}

pub fn move_baloon(baloon: &mut Baloon, weather: &Weather) {
    let baloon_latlng = Latlng { lat: baloon.lat, lng: baloon.lng };
    let wind_speed = weather.wind_speed;
    let wind_direction = weather.wind_deg;

    // Add random element to wind direction
    let random_direction: i16 = rand::thread_rng().gen_range(-10..10);
    let wind_direction_signed: i16 = (wind_direction as i16) + random_direction;
    let wind_direction: u16 = if wind_direction_signed < 0 {0} else {wind_direction_signed as u16};

    // Calculate new coordinates
    let new_latlng = calculate_new_position(baloon_latlng, wind_speed, wind_direction, 1.0);

    // Update baloon coordinates
    baloon.lat = new_latlng.lat;
    baloon.lng = new_latlng.lng;
}

fn calculate_new_position(latlng: Latlng, wind_speed: f32, wind_direction: u16, time_s: f32) -> Latlng {
    let distance_km = wind_speed * time_s / 1000.0; // Convert speed to distance in kilometers
    let bearing = wind_direction as f32;
    let bearing_rad = bearing.to_radians();

    let lat_rad = latlng.lat.to_radians();
    let lng_rad = latlng.lng.to_radians();

    let new_lat_rad = (lat_rad.sin() * distance_km.cos() + 
                      lat_rad.cos() * distance_km.sin() * bearing_rad.cos()).asin();
    let new_lng_rad = lng_rad + 
                     bearing_rad.sin().atan2(lat_rad.cos() * distance_km.sin() * bearing_rad.cos() -
                                         lat_rad.sin() * distance_km.cos());

    let new_lat = new_lat_rad.to_degrees();
    let new_lng = new_lng_rad.to_degrees();

    Latlng { lat: new_lat, lng: new_lng }
}