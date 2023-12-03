use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use once_cell::sync::Lazy;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub refresh_time_secs: u64,
    pub wind_speed_factor: f32,
    pub max_weather_distance_km: u16,
    pub max_cluster_distance_km: f32,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    read_config("config.yaml")
});

fn read_config<P: AsRef<Path>>(path: P) -> Config {
    let mut file = File::open(path).expect("Could not open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read config file");
    serde_yaml::from_str(&contents).expect("Could not parse config file")
}
