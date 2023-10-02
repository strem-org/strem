//! Application-specific configurations.
//!

use std::path::PathBuf;

/// Configuration information for Application.
///
/// This information does not capture the subcommands used---just flags, options,
/// and arguments.
#[derive(Debug)]
pub struct Configuration<'a> {
    /// The SpRE used for searching.
    pub pattern: &'a String,

    /// The data stream to search over. If this is `None`, then it is assumed
    /// the source is standard input.
    pub datastream: Option<Vec<PathBuf>>,

    /// Print the total number of matches found.
    pub count: bool,

    /// Maximum number of matches to search for.
    pub limit: Option<usize>,

    /// Formatting string.
    pub format: Option<&'a String>,

    /// Draw frames.
    pub draw: Option<&'a PathBuf>,
}
