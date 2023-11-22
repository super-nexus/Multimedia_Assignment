use serde::{Serialize, Deserialize};
use crate::baloon::model::Baloon;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct PoppedBaloon {
    pub baloon: Baloon,
    pub popped_at: i64
}
