use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Custom function to deserialize calibrated sensor intrinsic field.
///
/// Within the `calibrated_sensor.json` file, the field "intrinsic" has two
/// states: (1) an array that is empty or (2) an array that has three nested
/// arrays. However, the default deserializer expects the array to always be the
/// latter as it assumes the data to be well formed. Therefore, if the array is
/// empty, then the result is `None`. If the array is filled, then it is assumed
/// to be a 3x3 matrix.
fn deserialize_intrinsic<'de, D>(deserializer: D) -> Result<Option<[[f64; 3]; 3]>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::Array(a) => {
            if a.is_empty() {
                None
            } else {
                let mut intrinsic = Vec::new();
                for row in a.iter() {
                    intrinsic.push(
                        row.as_array()
                            .unwrap()
                            .iter()
                            .map(|x| x.as_f64().unwrap())
                            .collect::<Vec<f64>>()
                            .try_into()
                            .unwrap(),
                    );
                }

                Some(intrinsic.try_into().unwrap())
            }
        }
        _ => None,
    })
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Calibration {
    pub token: String,
    pub sensor_token: String,
    pub rotation: [f64; 4],
    pub translation: [f64; 3],
    #[serde(deserialize_with = "deserialize_intrinsic")]
    pub camera_intrinsic: Option<[[f64; 3]; 3]>,
}
