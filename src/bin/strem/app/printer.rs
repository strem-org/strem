//! Application printer.
//!

use std::error::Error;
use std::fmt;

#[cfg(feature = "export")]
use std::path::PathBuf;

use strem::config::Configuration;
use strem::datastream::frame::Frame;

#[cfg(feature = "export")]
pub mod imager;

pub struct Printer {}

impl Printer {
    /// Print a [`Match`].
    pub fn print(frames: &[Frame], config: &Configuration) -> Result<(), Box<dyn Error>> {
        let prefix = if let Some(path) = config.datastream {
            path.display().to_string()
        } else {
            String::from("")
        };

        println!(
            "{}: {:?}..{:?}",
            prefix,
            frames.first().unwrap().index,
            frames.last().unwrap().index + 1
        );

        #[cfg(feature = "export")]
        if let Some(outdir) = config.export {
            let imager = imager::Imager::new();

            let mut indir = PathBuf::new();

            if let Some(path) = config.datastream {
                if let Some(parent) = path.parent() {
                    indir.push(parent);
                }
            }

            for frame in frames.iter() {
                imager.draw(frame, &indir, outdir)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct PrinterError {
    msg: String,
}

impl From<&str> for PrinterError {
    fn from(msg: &str) -> Self {
        PrinterError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for PrinterError {
    fn from(msg: String) -> Self {
        PrinterError { msg }
    }
}

impl fmt::Display for PrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "printer: {}", self.msg)
    }
}

impl Error for PrinterError {}
