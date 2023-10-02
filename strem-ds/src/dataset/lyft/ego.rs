use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Ego {
    pub token: String,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    pub timestamp: f64,
}
