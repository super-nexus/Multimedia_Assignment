use serde::{Serialize, Deserialize};

const EARTH_RADIUS_KM: f32 = 6371.0; // Earth's radius in kilometers

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