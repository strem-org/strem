use std::collections::HashMap;
use std::path::PathBuf;

/// A sample record of object detections produced for a single frame.
///
/// This includes the labels and regions associated with such. Furthermore,
/// additional data related to the detection record is populated here.
#[derive(Clone, Debug)]
pub struct DetectionRecord {
    pub channel: String,
    pub timestamp: f64,
    pub image: Option<Image>,

    /// A mapping between labels and annotations (i.e., bounding boxes).
    pub annotations: HashMap<String, Vec<Annotation>>,
}

impl DetectionRecord {
    /// Create a new [`DetectionRecord`].
    pub fn new(channel: String, timestamp: f64, image: Option<Image>) -> Self {
        DetectionRecord {
            channel,
            timestamp,
            image,
            annotations: HashMap::new(),
        }
    }
}

/// An annotation of a label generated from a DNN.
///
/// This fundamentally includes the label, the region, and the confidence
/// ("score") of the resulting detection.
#[derive(Clone, Debug)]
pub struct Annotation {
    pub label: String,
    pub score: f64,
    pub bbox: BoundingBox,
}

impl Annotation {
    /// Create a new [`Annotation`] with associated data.
    pub fn new(label: String, score: f64, bbox: BoundingBox) -> Self {
        Annotation { label, score, bbox }
    }
}

/// An Axis-Aligned Bounding Box (AABB).
///
/// The selected representation of the AABB uses the major and minor coordinates
/// (i.e., the corners) to represent the rectangle.
#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    /// Create a new [`BoundingBox`] using min/max coordinates.
    pub fn new(min: Point, max: Point) -> Self {
        BoundingBox { min, max }
    }
}

/// A Z axis-aligned point (i.e., 2D).
#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Create a new [`Point`] with (x, y) coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

/// An interface to handle image metadata.
///
/// This includes source, dimensions, and any additional data that would be
/// associated with an image at this level.
#[derive(Clone, Debug)]
pub struct Image {
    pub source: ImageSource,
    pub width: f64,
    pub height: f64,
}

impl Image {
    /// Create a new [`Image`].
    pub fn new(source: ImageSource, width: f64, height: f64) -> Self {
        Image {
            source,
            width,
            height,
        }
    }
}

/// A interface to collect the image.
///
/// The image can be sourced from a file path, url, etc.
#[derive(Clone, Debug)]
pub enum ImageSource {
    File(PathBuf),
}
