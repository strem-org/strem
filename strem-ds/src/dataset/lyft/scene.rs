use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Scene {
    pub token: String,
    pub name: String,
    pub description: String,
    pub log_token: String,
    pub nbr_samples: i32,
    pub first_sample_token: String,
    pub last_sample_token: String,
}
