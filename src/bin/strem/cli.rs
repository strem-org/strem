//! Command-Line Interface configuration.
//!

use std::path::PathBuf;

use clap::{Arg, ArgAction, Command};

/// Build the Command-Line Interface application.
///
/// The structure of the command is organized follows: (1) parser settings,
/// (2) tool information, (3) positional arguments, (4) flags, and (5) options.
#[allow(clippy::let_and_return)]
pub fn build() -> Command {
    let cmd = Command::new(clap::crate_name!())
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
	     `strem --help` for more details of its usage.",
        )
        .after_long_help(
            "The use of `strem --help` prints a long and verbse overview. Use \
	     `strem -h` for less details.",
        )
        .arg(
            Arg::new("PATTERN")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(String))
                .help("A SpRE pattern used for searching"),
        )
        .arg(
            Arg::new("DATASTREAM")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(PathBuf))
                .help("The perception data stream to search over"),
        )
        .arg(
            Arg::new("channel")
                .short('c')
                .long("channel")
                .num_args(0..)
                .value_name("NAME")
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(String))
                .help("The channel to consider in the search"),
        )
        .arg(
            Arg::new("online")
                .short('o')
                .long("online")
                .action(ArgAction::SetTrue)
                .help("Use the online algorithm"),
        )
        .arg(
            Arg::new("max-count")
                .short('m')
                .long("max-count")
                .value_name("NUM")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(usize))
                .help("Stop searching after `NUM` matches found"),
        );

    #[cfg(feature = "export")]
    let cmd = cmd.arg(
        Arg::new("export")
            .short('x')
            .long("export")
            .value_name("DIR")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(PathBuf))
            .help("Export results to directory"),
    );

    cmd
}
