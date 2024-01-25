//! # Spatio-Temporal Regular Expression Matcher Tool
//!
//! Spatio-Temporal Regular Expression Matcher (STREM) is a
//! tool that provides pattern matching against annotated video datastreams
//! through the use of Spatial-based Regular Expressions (SpREs).
//!

use std::process;

mod app;
mod cli;

use crate::app::App;

fn main() {
    let app = App::new(cli::build().get_matches());

    match app.run() {
        Ok(..) => process::exit(0),
        Err(e) => {
            eprintln!("strem: error: {}", e);
            process::exit(1);
        }
    }
}
