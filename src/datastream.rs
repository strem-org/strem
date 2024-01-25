//! The ubiquitous perception stream structure for matching.
//!
//! This is the format from which all importers must import to.

use std::error::Error;
use std::fmt;

use self::frame::Frame;
use self::importer::DataImport;

pub mod frame;
pub mod importer;

/// An interface to interact with perception stream data.
///
/// It should be further noted that this interface provides basic mechanisms to
/// reading/writing of the stream regardless of offline/online application.
#[derive(Default)]
pub struct DataStream {
    pub frames: Vec<Frame>,

    /// The [`DataImport`] to retrieve [`Frame`] from.
    pub importer: Option<Box<dyn DataImport>>,

    /// A limit on the number of frames to keep in memory.
    pub capacity: Option<usize>,
}

impl DataStream {
    /// Create a new [`DataStream`] with the selected format.
    ///
    /// This function creates an empty [`DataStream`] instance that still must
    /// be further populated with frames.
    pub fn new() -> Self {
        DataStream {
            frames: Vec::new(),
            importer: None,
            capacity: None,
        }
    }

    /// Set the `capacity` of the [`DataStream`].
    pub fn capacity(mut self, size: usize) -> Self {
        self.capacity = Some(size);
        self
    }

    /// Set the [`DataImport`].
    pub fn importer(mut self, importer: Box<dyn DataImport>) -> Self {
        self.importer = Some(importer);
        self
    }

    /// Request the next frame from the [`DataImport`].
    pub fn request(
        &mut self,
        channels: &Option<Vec<String>>,
    ) -> Result<Option<Frame>, Box<dyn Error>> {
        if let Some(importer) = &mut self.importer {
            return importer.import(channels);
        }

        Err(Box::new(DataStreamError::from("missing data port")))
    }

    /// Insert a [`Frame`] at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the `index` > `self.frames.len()`.
    pub fn insert(&mut self, index: usize, frame: Frame) {
        self.frames.insert(index, frame);
    }

    /// Append a [`Frame`] at the end of the [`DataStream`].
    ///
    /// This is a shortcut method for [`Self::insert`] where the index is the length of
    /// the [`DataStream`].
    pub fn append(&mut self, frame: Frame) {
        self.insert(self.frames.len(), frame);
    }
}

impl fmt::Debug for DataStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DataStream")
            .field("frames", &self.frames)
            .field("capacity", &self.capacity)
            .finish()
    }
}

#[derive(Debug, Clone)]
struct DataStreamError {
    msg: String,
}

impl From<&str> for DataStreamError {
    fn from(msg: &str) -> Self {
        DataStreamError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for DataStreamError {
    fn from(msg: String) -> Self {
        DataStreamError { msg }
    }
}

impl fmt::Display for DataStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "datastream: {}", self.msg)
    }
}

impl Error for DataStreamError {}
