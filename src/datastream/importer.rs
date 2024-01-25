use std::error::Error;

use super::frame::Frame;

pub mod stremf;

/// A trait to that all data importers must implement.
pub trait DataImport {
    /// Parse the data into usable format.
    //
    // This is similar to the functionality of "loading" the raw text into memory
    // and preparing it for importing.
    fn load(&mut self) -> Result<(), Box<dyn Error>>;

    /// Import the next frame into a [`Frame`].
    fn import(&mut self, channels: &Option<Vec<String>>) -> Result<Option<Frame>, Box<dyn Error>>;
}
