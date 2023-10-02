use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Sample {
    pub token: String,
    pub timestamp: f64,
    pub scene_token: String,
    pub next: String,
    pub prev: String,
}
