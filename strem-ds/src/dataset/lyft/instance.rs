use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Instance {
    pub token: String,
    pub category_token: String,
    pub nbr_annotations: i32,
    pub first_annotation_token: String,
    pub last_annotation_token: String,
}
