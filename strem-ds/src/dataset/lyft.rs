mod annotation;
mod calibration;
mod category;
mod data;
mod ego;
mod instance;
mod sample;
mod scene;
mod sensor;

use std::{
    collections::HashMap,
    error::Error,
    fmt, fs,
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use nalgebra::{
    Quaternion, RowSVector as StaticRowVector, SMatrix as StaticMatrix,
    Translation as AlgebraTranslation, UnitQuaternion,
};
use serde::de::DeserializeOwned;
use strem::datastream::reader::{
    detection::{
        annotation::{Annotation, BoundingBox, Translation},
        category::Category,
        image::Image,
        Detection, Dimensions,
    },
    Sensor,
};

use self::{
    annotation::Annotation as LyftAnnotation, calibration::Calibration as LyftCalibration,
    category::Category as LyftCategory, data::Data as LyftData, ego::Ego as LyftEgo,
    instance::Instance as LyftInstance, sample::Sample as LyftSample, scene::Scene as LyftScene,
    sensor::Sensor as LyftSensor,
};
use super::DataSet as DataSetTrait;

type SampleToken = String;
type SceneToken = String;
type DataToken = String;
type InstanceToken = String;
type CategoryToken = String;
type EgoToken = String;
type CalibrationToken = String;
type SensorToken = String;

pub struct DataSet<'a> {
    pub rootdir: &'a Path,

    scenes: HashMap<SceneToken, LyftScene>,
    samples: HashMap<SampleToken, LyftSample>,
    annotations: HashMap<SampleToken, Vec<LyftAnnotation>>,
    datas_map: HashMap<SampleToken, Vec<LyftData>>,
    datas: HashMap<DataToken, LyftData>,
    instances: HashMap<InstanceToken, LyftInstance>,
    categories: HashMap<CategoryToken, (usize, LyftCategory)>,
    egos: HashMap<EgoToken, LyftEgo>,
    calibrations: HashMap<CalibrationToken, LyftCalibration>,
    sensors: HashMap<SensorToken, LyftSensor>,
}

impl<'a> DataSet<'a> {
    pub fn new(rootdir: &'a Path) -> Self {
        Self {
            rootdir,
            scenes: HashMap::new(),
            samples: HashMap::new(),
            annotations: HashMap::new(),
            datas_map: HashMap::new(),
            datas: HashMap::new(),
            instances: HashMap::new(),
            categories: HashMap::new(),
            egos: HashMap::new(),
            calibrations: HashMap::new(),
            sensors: HashMap::new(),
        }
    }
    /// Import JSON-based data from the Lyft Dataset.
    ///
    /// # Type Parameters
    /// - `T`: The type to deserialize into.
    ///
    /// When implementing types, it is only necessary for them to both
    /// serializable and deserializable according to the
    /// [`serde`](https://docs.rs/serde/latest/serde/index.html) crate.
    ///
    /// # Fallback
    /// Currently, the Lyft Level 5 Perception Dataset uses a non-standard
    /// version of JSON where some fields are attributed with `NaN`. According
    /// to [ECMA-404](https://www.ecma-international.org/publications-and-standards/standards/ecma-404/),
    /// `Infinity` and `NaN` are not supported. Therefore, to resolve this issue and
    /// support some productions from ES5, [`json5`] is used when [`serde`] fails
    /// to load the data.
    fn import<T>(&self, filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + 'static,
    {
        let mut path = PathBuf::from(self.rootdir);

        // extend path to data
        path.push("train_data");
        path.push(filename);

        let f = File::open(&path)?;

        let reader = BufReader::new(f);

        if let Ok(data) = serde_json::from_reader(reader) {
            Ok(data)
        } else {
            if let Ok(s) = fs::read_to_string(&path) {
                if let Ok(data) = json5::from_str(s.as_str()) {
                    return Ok(data);
                } else {
                    return Err(Box::new(LyftError::from(format!(
                        "unable to read data from {} (perhaps formatted incorrectly)",
                        path.to_str().unwrap()
                    ))));
                }
            }

            Err(Box::new(LyftError::from(format!(
                "unable to open {}",
                path.to_str().unwrap()
            ))))
        }
    }
}

impl<'a> DataSetTrait for DataSet<'a> {
    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.scenes = self
            .import::<LyftScene>("scene.json")?
            .into_iter()
            .map(|x| (x.token.clone(), x))
            .collect();

        self.samples = self
            .import::<LyftSample>("sample.json")?
            .into_iter()
            .map(|x| (x.token.clone(), x))
            .collect();

        for annotation in self.import::<LyftAnnotation>("sample_annotation.json")? {
            self.annotations
                .entry(annotation.sample_token.clone())
                .or_insert_with(Vec::new)
                .push(annotation);
        }

        for data in self
            .import::<LyftData>("sample_data.json")?
            .into_iter()
            .filter(|x| x.is_key_frame)
        {
            self.datas_map
                .entry(data.sample_token.clone())
                .or_insert_with(Vec::new)
                .push(data);
        }

        self.datas = self
            .import::<LyftData>("sample_data.json")?
            .into_iter()
            .map(|x| (x.token.clone(), x))
            .collect();

        self.instances = self
            .import::<LyftInstance>("instance.json")?
            .into_iter()
            .map(|x| (x.token.clone(), x))
            .collect();

        self.categories = self
            .import::<LyftCategory>("category.json")?
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x.token.clone(), (i, x)))
            .collect();

        self.egos = self
            .import::<LyftEgo>("ego_pose.json")?
            .into_iter()
            .map(|x| (x.token.clone(), x))
            .collect();

        self.calibrations = self
            .import::<LyftCalibration>("calibrated_sensor.json")?
            .into_iter()
            .map(|x| (x.token.clone(), x))
            .collect();

        self.sensors = self
            .import::<LyftSensor>("sensor.json")?
            .into_iter()
            .map(|x| (x.token.clone(), x))
            .collect();

        Ok(())
    }

    fn export(&self) -> Result<(), Box<dyn Error>> {
        for (token, scene) in self.scenes.iter() {
            // Collect the set of initial samples that are key frames.
            //
            // The key frames are of significance here because they represent
            // a frame where the time difference between it and the set of
            // sensor recordings are sufficiently close.
            //
            // Note: The initial samples are a list of samples that correspond to
            // each sensor. For example, this (in most cases) will return 10
            // total samples each corresponding to a different sensor all
            // recorded at the "same" frame.
            //
            // Still have doubts? Print out the initials to see!
            let initials: Vec<&LyftData> =
                match self.datas_map.get(scene.first_sample_token.as_str()) {
                    Some(x) => x.iter().filter(|x| x.is_key_frame).collect(),
                    None => {
                        return Err(Box::new(LyftError::from(format!(
                            "scene ({}) does not have a valid first sample",
                            token
                        ))))
                    }
                };

            // In this loop, if an item is not properly found in the dataset,
            // then it is silently ignored and the next element in the intial set
            // of samples is used.
            for initial in initials {
                // Follow this initial in time.
                let mut data = initial;

                let calibration = match self.calibrations.get(data.calibrated_sensor_token.as_str())
                {
                    Some(x) => x,
                    None => continue,
                };

                let sensor = match self.sensors.get(calibration.sensor_token.as_str()) {
                    Some(x) => x,
                    None => continue,
                };

                let view = if let Some(view) = calibration.camera_intrinsic {
                    StaticMatrix::<f64, 3, 3>::from_rows(&[
                        StaticRowVector::<f64, 3>::from(view[0]),
                        StaticRowVector::<f64, 3>::from(view[1]),
                        StaticRowVector::<f64, 3>::from(view[2]),
                    ])
                } else {
                    continue;
                };

                // let mut frames: Vec<Frame> = Vec::new();
                // let mut index: usize = 0;

                let mut detections: Vec<Detection> = Vec::new();

                while !data.next.is_empty() {
                    let sensor_data = Sensor::Camera;
                    let channel_data = format!("{}/{}", token, sensor.channel);
                    let image_data = Image {
                        dimensions: Dimensions {
                            width: data.width.unwrap(),
                            height: data.height.unwrap(),
                        },
                        source: strem::datastream::reader::detection::image::ImageSource::File {
                            path: PathBuf::from(&data.filename),
                        },
                    };

                    let categories_data: HashMap<String, Category> = self
                        .categories
                        .iter()
                        .map(|(_, (id, category))| {
                            (
                                category.name.clone(),
                                Category {
                                    id: *id,
                                    name: category.name.clone(),
                                    supercategory: None,
                                },
                            )
                        })
                        .collect();

                    let sample = match self.samples.get(data.sample_token.as_str()) {
                        Some(x) => x,
                        None => continue,
                    };

                    // let mut frame = Frame::new(sample.timestamp, index, data.filename.clone());
                    // let mut objects: Vec<Object> = Vec::new();

                    let annotations = match self.annotations.get(sample.token.as_str()) {
                        Some(x) => x,
                        None => continue,
                    };

                    let mut annotation_map = HashMap::new();

                    for annotation in annotations {
                        let instance = match self.instances.get(annotation.instance_token.as_str())
                        {
                            Some(x) => x,
                            None => continue,
                        };

                        let class = match self.categories.get(instance.category_token.as_str()) {
                            Some(x) => x,
                            None => continue,
                        };

                        let ego = match self.egos.get(data.ego_pose_token.as_str()) {
                            Some(x) => x,
                            None => continue,
                        };

                        // Transform the annotation with respect to the
                        // specified camera sensor.
                        //
                        // This is done in two transformations: (1) transform
                        // w.r.t. to the ego vehicle, and (2) transform
                        // w.r.t. to the camera sensor.
                        let annotation = annotation
                            .clone()
                            .transform(
                                AlgebraTranslation::<f64, 3>::new(
                                    ego.translation[0],
                                    ego.translation[1],
                                    ego.translation[2],
                                )
                                .inverse(),
                                UnitQuaternion::from_quaternion(Quaternion::new(
                                    ego.rotation[0],
                                    ego.rotation[1],
                                    ego.rotation[2],
                                    ego.rotation[3],
                                ))
                                .inverse(),
                            )
                            .transform(
                                AlgebraTranslation::<f64, 3>::new(
                                    calibration.translation[0],
                                    calibration.translation[1],
                                    calibration.translation[2],
                                )
                                .inverse(),
                                UnitQuaternion::from_quaternion(Quaternion::new(
                                    calibration.rotation[0],
                                    calibration.rotation[1],
                                    calibration.rotation[2],
                                    calibration.rotation[3],
                                ))
                                .inverse(),
                            );

                        // Create [`Object`] iff the annotation exists
                        // within the 2D projected image.
                        if annotation.inside(
                            view,
                            (image_data.dimensions.width, image_data.dimensions.height),
                        ) {
                            let minmax = annotation.projection(view);

                            let xmin = minmax.row(0).iter().copied().fold(f64::NAN, f64::min);
                            let ymin = minmax.row(1).iter().copied().fold(f64::NAN, f64::min);

                            let xmax = minmax.row(0).iter().copied().fold(f64::NAN, f64::max);
                            let ymax = minmax.row(1).iter().copied().fold(f64::NAN, f64::max);

                            let bbox_width = (xmax - xmin).abs();
                            let bbox_height = (ymax - ymin).abs();

                            let center_x = xmin + (bbox_width / 2.0);
                            let center_y = ymin + (bbox_height / 2.0);

                            let annotation_data = Annotation {
                                category: class.0,
                                score: 1.0,
                                bbox: BoundingBox {
                                    dimensions: Dimensions {
                                        width: bbox_width,
                                        height: bbox_height,
                                    },
                                    translation: Translation {
                                        x: center_x,
                                        y: center_y,
                                    },
                                },
                            };

                            annotation_map
                                .entry(class.0)
                                .or_insert(Vec::new())
                                .push(annotation_data);

                            // objects.push(Object::new(
                            //     sample.timestamp,
                            //     class.name.clone(),
                            //     BoundingBox::from(annotation.projection(view)),
                            // ));
                        }
                    }

                    // for object in objects {
                    //     match frame.detections.entry(object.class.clone()) {
                    //         Entry::Vacant(v) => {
                    //             let v = v.insert(Vec::new());
                    //             v.push(object);
                    //         }
                    //         Entry::Occupied(mut o) => o.get_mut().push(object),
                    //     }
                    // }

                    // frames.push(frame);

                    // Follow the next frame from this sensor.
                    data = match self.datas.get(data.next.as_str()) {
                        Some(x) => x,
                        None => {
                            return Err(Box::new(LyftError::from(format!(
                                "cannot find next sample data from current sample data ({})",
                                data.token
                            ))))
                        }
                    };

                    detections.push(Detection {
                        sensor: sensor_data,
                        channel: channel_data,
                        image: Some(image_data),
                        categories: categories_data,
                        annotations: annotation_map,
                    });

                    // index += 1;
                }

                let mut outfile = BufWriter::new(
                    fs::File::create(format!("{}_{}.json", scene.token, sensor.channel)).unwrap(),
                );

                for detection in detections {
                    outfile
                        .write(serde_json::to_string(&detection).unwrap().as_bytes())
                        .expect("writtening");
                }

                outfile.flush().unwrap();

                // serde_json::to_writer(outfile, &detections).expect("cant write..");

                // println!("{:?}", serde_json::to_string(&detections));

                // current.channels.insert(sensor.channel.clone(), frames);
            }

            // stream.push(current);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct LyftError {
    msg: String,
}

impl From<&str> for LyftError {
    fn from(msg: &str) -> Self {
        LyftError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for LyftError {
    fn from(msg: String) -> Self {
        LyftError { msg }
    }
}

impl fmt::Display for LyftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lyft: {}", self.msg)
    }
}

impl Error for LyftError {}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOTDIR: &str = env!("CARGO_MANIFEST_DIR");
    const DATADIR: &str = "data/scene";

    #[test]
    fn instantiate_dataset() {
        let mut path = PathBuf::from(self::ROOTDIR);
        path.push(self::DATADIR);

        DataSet::new(&path);
    }

    #[test]
    fn load_dataset() {
        let mut path = PathBuf::from(self::ROOTDIR);
        path.push(self::DATADIR);

        let mut dataset = DataSet::new(&path);
        dataset.load().expect("loading error...");
    }
}
