//! Application-specific configurations.
//!

use std::path::PathBuf;

/// Configuration information for Application.
///
/// This information does not capture the subcommands used---just flags, options,
/// and arguments.
pub struct Configuration<'a> {
    /// The SpRE used for searching.
    pub pattern: &'a String,

    /// The data stream to search over. If this is `None`, then it is assumed
    /// the source is standard input.
    pub datastream: Option<&'a PathBuf>,

    /// Use the online algorithm.
    pub online: bool,

    /// A collection of channels to import.
    pub channels: Option<Vec<String>>,

    /// Maximum number of matches to search for.
    pub limit: Option<usize>,

    /// Draw frames.
    #[cfg(feature = "export")]
    pub export: Option<&'a PathBuf>,
}
