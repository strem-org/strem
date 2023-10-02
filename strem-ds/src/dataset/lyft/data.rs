use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    pub token: String,
    pub is_key_frame: bool,
    pub timestamp: f64,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub fileformat: String,
    pub filename: String,
    pub ego_pose_token: String,
    pub sample_token: String,
    pub calibrated_sensor_token: String,
    pub next: String,
    pub prev: String,
}
