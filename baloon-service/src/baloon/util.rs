use std::collections::HashMap;
use crate::weather::model::Weather;
use crate::baloon::model::{Baloon, Latlng};
use rand::Rng;

const MAX_DISTANCE_KM: f32 = 10.0; // Define a maximum distance for clustering in kilometers

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
    let bearing_rad = (wind_direction as f32).to_radians();

    let earth_radius_km = 6371.0; // Radius of the Earth in kilometers
    let distance_rad = distance_km / earth_radius_km;

    let lat_rad = latlng.lat.to_radians();
    let lng_rad = latlng.lng.to_radians();

    let new_lat_rad = (lat_rad.sin() * distance_rad.cos() + lat_rad.cos() * distance_rad.sin() * bearing_rad.cos()).asin();
    let new_lng_rad = lng_rad + f32::atan2(
        bearing_rad.sin() * distance_rad.sin() * lat_rad.cos(),
        distance_rad.cos() - lat_rad.sin() * new_lat_rad.sin()
    );

    let new_lat = new_lat_rad.to_degrees();
    let new_lng = new_lng_rad.to_degrees();

    Latlng { lat: new_lat, lng: new_lng }
}