use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Dimensions;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub dimensions: Dimensions,
    pub source: ImageSource,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ImageSource {
    #[serde(alias = "file")]
    File { path: PathBuf },
}
