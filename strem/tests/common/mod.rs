use std::path::PathBuf;

use strem::config::{Configuration, Flag};

pub fn config(pattern: &str, sensor: &str) -> Configuration {
    let mut dataset = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dataset.push("data/scene");

    Configuration {
        pattern: String::from(pattern),
        datastreams: vec![dataset],
        alphabet: ('a'..='z').into_iter().collect(),
        count: Flag {
            active: false,
            index: None,
        },
        dataset: String::from("L5"),
        dimension: (1920, 1080),
        sensors: vec![sensor.to_string()],
        export: Flag {
            active: false,
            index: None,
        },
        limit: Some(10),
        scenes: None,
        format: String::from("%f:%s:%c:%m"),
    }
}
