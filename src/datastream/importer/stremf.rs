use std::error::Error;
use std::fmt;
use std::io::BufReader;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::datastream::frame::sample::detections::{
    Annotation, BoundingBox, DetectionRecord, Image, ImageSource, Point,
};
use crate::datastream::frame::sample::Sample;
use crate::datastream::frame::Frame;

use super::DataImport;

/// A reader for importing STREM-formatted data.
pub struct DataImporter {
    path: PathBuf,

    data: Option<StremFormat>,
    index: usize,
}

impl DataImporter {
    /// Create a new [`DataImporter`].
    pub fn new(path: PathBuf) -> Self {
        DataImporter {
            path,
            data: None,
            index: 0,
        }
    }
}

impl DataImport for DataImporter {
    /// From the provided source, load the data.
    ///
    /// This is a pre-step before any actual data can be loaded into the format
    /// to be searched over (e.g., setting up caches, etc).
    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        // The data should only be loaded once.
        //
        // This check is performed to avoid redundant I/O operations from the
        // system.
        if self.data.is_none() {
            let reader = BufReader::new(std::fs::File::open(&self.path)?);
            let data: StremFormat = serde_json::from_reader(reader)?;

            if data.version != "1.0.0" {
                return Err(Box::new(DataImporterError::from("expected v1.0.0")));
            }

            self.data = Some(data);
        }

        Ok(())
    }

    /// Import the next frame into a [`Frame`].
    ///
    /// This function converts a STREM-formatted frame into the ubiquitous
    /// [`Frame`] format for searching.
    fn import(&mut self, channels: &Option<Vec<String>>) -> Result<Option<Frame>, Box<dyn Error>> {
        // Convert next frame from [`StremFormat`] into a [`Frame`].
        //
        // This consists of molding the imported data to fit the structure of a
        // [`Frame`], accordingly.
        if let Some(data) = &self.data {
            if self.index < data.frames.len() {
                let f = &data.frames[self.index];
                self.index += 1;

                let mut frame = Frame::new(f.index, f.timestamp);

                // For each sample, collect the set of relevant annotations and
                // add to sample map of [`Frame`].
                for s in f.samples.iter() {
                    if let Some(channels) = channels {
                        if !channels.contains(&s.channel) {
                            // The channel from the data is not in the specified
                            // channels. Therefore, we skip it.
                            continue;
                        }
                    }

                    let mut record = DetectionRecord::new(
                        s.channel.clone(),
                        s.timestamp,
                        Some(Image::new(
                            ImageSource::File(PathBuf::from(&s.image.path)),
                            s.image.dimensions.width,
                            s.image.dimensions.height,
                        )),
                    );

                    // Add annotations to the [`DetectionRecord`].
                    for a in s.annotations.iter() {
                        let bbox = BoundingBox::new(
                            Point::new(a.bbox.x, a.bbox.y),
                            Point::new(a.bbox.x + a.bbox.w, a.bbox.y + a.bbox.h),
                        );

                        record
                            .annotations
                            .entry(a.class.clone())
                            .or_default()
                            .push(Annotation::new(a.class.clone(), a.score, bbox));
                    }

                    frame.samples.push(Sample::ObjectDetection(record));
                }

                return Ok(Some(frame));
            }
        }

        Ok(None)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StremFormat {
    pub version: String,
    pub frames: Vec<StremFrame>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StremFrame {
    pub index: usize,
    pub timestamp: f64,
    pub samples: Vec<StremSample>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StremSample {
    pub channel: String,
    pub timestamp: f64,
    pub image: StremImage,
    pub annotations: Vec<StremAnnotation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StremImage {
    pub path: String,
    pub dimensions: StremDimension,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StremDimension {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StremAnnotation {
    pub class: String,
    pub score: f64,
    pub bbox: StremBoundingBox,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StremBoundingBox {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Clone)]
struct DataImporterError {
    msg: String,
}

impl From<&str> for DataImporterError {
    fn from(msg: &str) -> Self {
        DataImporterError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for DataImporterError {
    fn from(msg: String) -> Self {
        DataImporterError { msg }
    }
}

impl fmt::Display for DataImporterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "importer: stremf: {}", self.msg)
    }
}

impl Error for DataImporterError {}
