//! STREM application.
//!

use std::error::Error;
use std::fmt;

use clap::ArgMatches;
use strem::config::Configuration;
use strem::controller::Controller;
use strem::datastream::importer::stremf::DataImporter;
use strem::datastream::importer::DataImport;
use strem::datastream::DataStream;

use self::printer::Printer;

mod printer;

pub struct App {
    matches: ArgMatches,
}

impl App {
    pub fn new(matches: ArgMatches) -> Self {
        Self { matches }
    }

    /// Run the strem application.
    ///
    /// This method is responsible for selecting what to run with what
    /// [`Configuration`] based on the arguments, options, and (most importantly)
    /// the subcommand(s).
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let config = self.configure()?;

        // Load data into memory.
        let mut importer = DataImporter::new(config.datastream.unwrap().clone());
        importer.load()?;

        let datastream = DataStream::new().importer(Box::new(importer));

        // Set up and run [`Controller`].
        let controller = Controller::new(&config, Some(Printer::print));
        controller.run(datastream)?;

        Ok(())
    }

    /// Create a [`Configuration`] from the CLI arguments.
    fn configure(&self) -> Result<Configuration, Box<dyn Error>> {
        #[cfg(feature = "export")]
        return Ok(Configuration {
            pattern: self.matches.get_one("PATTERN").unwrap(),
            datastream: self.matches.get_one("DATASTREAM"),
            online: self.matches.get_flag("online"),
            limit: self.matches.get_one("max-count").copied(),
            export: self.matches.get_one("export"),
            channels: self
                .matches
                .get_many::<String>("channel")
                .map(|channels| channels.cloned().collect::<Vec<String>>()),
        });

        #[cfg(not(feature = "export"))]
        Ok(Configuration {
            pattern: self.matches.get_one("PATTERN").unwrap(),
            datastream: self.matches.get_one("DATASTREAM"),
            online: self.matches.get_flag("online"),
            limit: self.matches.get_one("max-count").copied(),
            channels: self
                .matches
                .get_many::<String>("channel")
                .map(|channels| channels.cloned().collect::<Vec<String>>()),
        })
    }
}

#[derive(Debug, Clone)]
struct AppError {
    msg: String,
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError { msg }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "app: {}", self.msg)
    }
}

impl Error for AppError {}
