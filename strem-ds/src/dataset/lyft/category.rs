use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Category {
    pub token: String,
    pub name: String,
}
