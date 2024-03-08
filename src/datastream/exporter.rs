use std::{error::Error, path::Path};

use super::frame::Frame;

pub mod stremf;

/// A trait that all data exporters must implement.
pub trait DataExport {
    /// Export the set of [`Frame`] to file.
    fn export(&self, frames: &[Frame], path: &Path) -> Result<(), Box<dyn Error>>;
}
