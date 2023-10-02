pub mod annotation;
pub mod category;
pub mod image;

use std::collections::HashMap;

use super::Sensor;
use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

use self::{annotation::Annotation, category::Category, image::Image};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Detection {
    pub sensor: Sensor,
    pub channel: String,
    pub image: Option<Image>,

    #[serde(
        deserialize_with = "self::category::deserialize",
        serialize_with = "self::serialize_categories_map"
    )]
    pub categories: HashMap<String, Category>,

    #[serde(
        deserialize_with = "self::annotation::deserialize",
        serialize_with = "self::serialize_annotation_map"
    )]
    pub annotations: HashMap<usize, Vec<Annotation>>,
}

pub fn serialize_categories_map<S>(
    map: &HashMap<String, Category>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut categories = Vec::new();

    for cat in map.values() {
        categories.push(cat);
    }

    let mut seq = serializer.serialize_seq(Some(categories.len()))?;
    for c in categories.into_iter() {
        seq.serialize_element(&c)?;
    }

    seq.end()
}

pub fn serialize_annotation_map<S>(
    map: &HashMap<usize, Vec<Annotation>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut annotations = Vec::new();

    for anns in map.values() {
        annotations.extend(anns);
    }

    let mut seq = serializer.serialize_seq(Some(annotations.len()))?;
    for a in annotations.into_iter() {
        seq.serialize_element(&a)?;
    }

    seq.end()
}
