use self::detections::DetectionRecord;

pub mod detections;

/// A kind of data captured by a channel of the perception sytem.
///
/// This categorization usually reflects the various sensors that are affixed to
/// perception systems such as LiDAR, radar, etc. However, it also captures
/// results provided by processes down the pipeline or sensor fusion ouputs such
/// as road lines, object detections, etc.
#[derive(Clone, Debug)]
pub enum Sample {
    /// A sample of object detection(s).
    ObjectDetection(DetectionRecord),
}
