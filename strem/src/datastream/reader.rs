use std::{error::Error, io::Read};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::detection::Detection;

pub mod detection;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Sensor {
    #[serde(rename = "camera")]
    Camera,
}

#[derive(Clone, Debug)]
pub enum Sample {
    ObjectDetection(Detection),
}

pub trait DataRead {
    fn next(&mut self) -> Result<Option<Sample>, Box<dyn Error>>;
}

pub struct DataReader<R: Read> {
    pub reader: R,
}

impl<R: Read> DataRead for DataReader<R> {
    fn next(&mut self) -> Result<Option<Sample>, Box<dyn Error>> {
        let mut streamer =
            serde_json::Deserializer::from_reader(&mut self.reader).into_iter::<Value>();

        match streamer.next() {
            None => Ok(None),
            Some(result) => {
                let sample = self.sample(result?)?;
                Ok(Some(sample))
            }
        }
    }
}

impl<R: Read> DataReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    fn sample(&self, value: Value) -> Result<Sample, Box<dyn Error>> {
        let sample = if let Some(object) = value.as_object() {
            if let Some(sensor) = object.get("sensor") {
                if let Some(object) = sensor.as_object() {
                    if let Some(kind) = object.get("type") {
                        if let Some(string) = kind.as_str() {
                            match string {
                                "camera" => {
                                    Sample::ObjectDetection(DataReader::<R>::detection(value)?)
                                }
                                _ => panic!("json: unsupported sensor type"),
                            }
                        } else {
                            panic!("json: not string")
                        }
                    } else {
                        panic!("json: missing `type`")
                    }
                } else {
                    panic!("json: expected object")
                }
            } else {
                panic!("json: missing field `sensor`")
            }
        } else {
            panic!("json: expected object")
        };

        Ok(sample)
    }

    fn detection(value: Value) -> Result<Detection, serde_json::Error> {
        serde_json::from_value(value)
    }
}
