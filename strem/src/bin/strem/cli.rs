//! Command-Line Interface configuration.
//!

use std::path::PathBuf;

use clap::{AppSettings, Arg, ArgAction, Command};

/// Build the Command-Line Interface application.
///
/// The structure of the command is organized follows: (1) parser settings,
/// (2) tool information, (3) positional arguments, (4) flags, and (5) options.
pub fn build() -> Command<'static> {
    Command::new(clap::crate_name!())
        .global_setting(AppSettings::DeriveDisplayOrder)
        .help_expected(true)
        .dont_collapse_args_in_usage(true)
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .long_about(
            "Spatio-Temporal Regular Expression Matching (STREM) tool performs \
	     pattern matching against a perception datastream through the use \
	     of Spatial-based Regular Expressions (SpREs).",
        )
        .after_help(
            "The use of `strem -h` prints a short and concise overview. Use \
	     `strem --help` for more details.",
        )
        .after_long_help(
            "The use of `strem --help` prints a long and verbse overview. Use \
	     `strem -h` for less details.",
        )
        .arg(
            Arg::new("PATTERN")
                .required(true)
                .takes_value(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(String))
                .help("A SpRE pattern used for searching"),
        )
        .arg(
            Arg::new("DATASTREAM")
                .takes_value(true)
                .multiple_values(true)
                .value_parser(clap::value_parser!(PathBuf))
                .help("The perception data stream(s) to search over"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Print the number of matches found"),
        )
        .arg(
            Arg::new("format")
                .short('F')
                .long("format")
                .takes_value(true)
                .value_name("fmt")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(String))
                .help("Format the results of a found match"),
        )
        .arg(
            Arg::new("limit")
                .short('l')
                .long("limit")
                .takes_value(true)
                .value_name("num")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(usize))
                .help("Stop searching after `num` matches found"),
        )
        .arg(
            Arg::new("draw")
                .short('d')
                .long("draw")
                .takes_value(true)
                .value_name("path")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(PathBuf))
                .help("Draw frame matches to provided `path`."),
        )
}
