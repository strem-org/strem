//! STREM application.
//!

mod printer;

use std::error::Error;
use std::io::BufReader;
use std::path::PathBuf;
use std::{error, fmt, fs};

use clap::ArgMatches;

use strem::compiler::Compiler;
use strem::config::Configuration;
use strem::datastream::frame::Frame;
use strem::datastream::reader::{DataRead, DataReader};
use strem::datastream::DataStream;
use strem::matcher::Matcher;
use strem::symbolizer::ast::SymbolicAbstractSyntaxTree;

use self::printer::Printer;

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
    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        let config = self.configure();

        // Compile SpRE into IR.
        let ast = Compiler::new().compile(config.pattern)?;

        // Initialize data stream.
        //
        // The Rust design pattern used here is formally known as a
        // 'Trait Object'. This is necessary as we utilize the same `DataStream`
        // interface with potentially different sources that are not known until
        // runtime.
        //
        // In short, we used dynamic typing.
        match &config.datastream {
            Some(paths) => {
                for path in paths {
                    let reader = BufReader::new(fs::File::open(path)?);
                    let reader = Box::new(DataReader::new(reader));

                    self.search(&ast, reader, &config, Some(path))?;
                }
            }
            None => {
                let reader = BufReader::new(std::io::stdin());
                let reader = Box::new(DataReader::new(reader));

                self.search(&ast, reader, &config, None)?;
            }
        };

        Ok(())
    }

    fn search(
        &self,
        ast: &SymbolicAbstractSyntaxTree,
        reader: Box<dyn DataRead>,
        config: &Configuration,
        path: Option<&PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        if config.datastream.is_some() {
            self.offline(ast, reader, config, path)?;
        } else {
            self.online(ast, reader, config)?;
        }

        Ok(())
    }

    fn offline(
        &self,
        ast: &SymbolicAbstractSyntaxTree,
        mut reader: Box<dyn DataRead>,
        config: &Configuration,
        path: Option<&PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        let mut count = 0;
        let mut mcount = 0;

        let mut datastream = DataStream::new();

        while let Some(sample) = reader.next()? {
            let mut frame = Frame::new(count);
            frame.samples.push(sample);
            count += 1;

            datastream.frames.push(frame);
        }

        let mut printer = Printer::new(config, path);
        let mut matcher = Matcher::from(ast);

        let mut start = datastream.frames.len();

        while start > 0 {
            let m = matcher.run(&datastream.frames[..start]);
            matcher.reset();

            if let Some(m) = m {
                mcount += 1;

                if let Some(limit) = config.limit {
                    if mcount > limit {
                        break;
                    }
                }

                printer.print(&m)?;
                start = m.frames.first().unwrap().index;
            } else {
                start -= 1;
            }
        }

        Ok(())
    }

    fn online(
        &self,
        ast: &SymbolicAbstractSyntaxTree,
        mut reader: Box<dyn DataRead>,
        config: &Configuration,
    ) -> Result<(), Box<dyn Error>> {
        let mut count = 0;
        let mut mcount = 0;

        let mut datastream = DataStream::new();

        let mut printer = Printer::new(config, None);
        let mut matcher = Matcher::from(ast);

        while let Some(sample) = reader.next()? {
            let mut frame = Frame::new(count);
            frame.samples.push(sample);
            count += 1;

            datastream.frames.push(frame);

            let m = matcher.run(&datastream.frames);
            matcher.reset();

            if let Some(m) = m {
                mcount += 1;

                if let Some(limit) = config.limit {
                    if mcount > limit {
                        break;
                    }
                }

                printer.print(&m)?;
            }
        }

        Ok(())
    }

    fn configure(&self) -> Configuration {
        let streams = self
            .matches
            .get_many::<PathBuf>("DATASTREAM")
            .map(|streams| streams.cloned().collect::<Vec<PathBuf>>());

        Configuration {
            pattern: self.matches.get_one("PATTERN").unwrap(),
            datastream: streams,
            count: self.matches.get_flag("count"),
            limit: self.matches.get_one("limit").copied(),
            format: self.matches.get_one("format"),
            draw: self.matches.get_one("draw"),
        }
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

impl error::Error for AppError {}
