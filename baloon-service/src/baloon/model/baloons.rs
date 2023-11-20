use serde::{Serialize, Deserialize};
use crate::baloon::model::Baloon;

#[derive(Serialize, Deserialize, Debug)]
pub struct Baloons {
    pub baloons: Vec<Baloon>
}