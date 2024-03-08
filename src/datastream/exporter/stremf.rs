use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;

use crate::datastream::frame::sample::detections::ImageSource;
use crate::datastream::frame::sample::Sample;
use crate::datastream::frame::Frame;
use crate::datastream::importer::stremf::{
    StremAnnotation, StremBoundingBox, StremDimension, StremFormat, StremFrame, StremImage,
    StremSample,
};

use super::DataExport;

const STREMF_VERSION: &str = "1.0.0";

/// A reader for importing STREM-formatted data.
#[derive(Default)]
pub struct DataExporter {}

impl DataExporter {
    /// Create a new [`DataExporter`].
    pub fn new() -> Self {
        DataExporter {}
    }
}

impl DataExport for DataExporter {
    fn export(&self, frames: &[Frame], path: &Path) -> Result<(), Box<dyn Error>> {
        let infile = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .or(Err(Box::new(DataExporterError::from(format!(
                "unable to write to `{}`",
                path.display()
            )))))?;

        let mut sformat = StremFormat {
            version: String::from(STREMF_VERSION),
            frames: Vec::new(),
        };

        for frame in frames {
            let mut s = Vec::new();
            for sample in frame.samples.iter() {
                match sample {
                    Sample::ObjectDetection(record) => {
                        let i = record.image.as_ref().map(|image| StremImage {
                            path: match &image.source {
                                ImageSource::File(path) => String::from(path.to_str().unwrap()),
                            },
                            dimensions: StremDimension {
                                width: image.width,
                                height: image.height,
                            },
                        });

                        let mut a = Vec::new();
                        for annotations in record.annotations.values() {
                            for annotation in annotations {
                                a.push(StremAnnotation {
                                    class: annotation.label.clone(),
                                    score: annotation.score,
                                    bbox: StremBoundingBox {
                                        x: annotation.bbox.min.x,
                                        y: annotation.bbox.min.y,
                                        w: annotation.bbox.max.x - annotation.bbox.min.x,
                                        h: annotation.bbox.max.y - annotation.bbox.min.y,
                                    },
                                });
                            }
                        }

                        s.push(StremSample {
                            channel: record.channel.clone(),
                            timestamp: record.timestamp,
                            image: i.unwrap(),
                            annotations: a,
                        });
                    }
                }
            }

            sformat.frames.push(StremFrame {
                index: frame.index,
                timestamp: frame.timestamp,
                samples: s,
            });
        }

        serde_json::to_writer(BufWriter::new(infile), &sformat)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct DataExporterError {
    msg: String,
}

impl From<&str> for DataExporterError {
    fn from(msg: &str) -> Self {
        DataExporterError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for DataExporterError {
    fn from(msg: String) -> Self {
        DataExporterError { msg }
    }
}

impl fmt::Display for DataExporterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "exporter: stremf: {}", self.msg)
    }
}

impl Error for DataExporterError {}
