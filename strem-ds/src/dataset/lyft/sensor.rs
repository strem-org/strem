use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Sensor {
    pub token: String,
    pub modality: String,
    pub channel: String,
}
