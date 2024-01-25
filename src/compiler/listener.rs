//! Error listener for reporting and exiting.
//!
//! This error listener is contextualized for the [compiler](../index.html). Therefore, its
//! usage outside the scope of this is not tested.

use std::process;

/// Interface to handle errors.
///
/// This is a general error listener that can be attached to any process that
/// supports error listeners. Therefore, for more specific error messages
/// and better support, a particular error listener should be implemented and
/// attached to the process, accordingly.
#[derive(Default)]
pub struct ErrorListener {}

impl ErrorListener {
    pub fn new() -> Self {
        ErrorListener {}
    }

    /// Print an error to stderr.
    pub fn report(&self, e: String) {
        eprintln!("listener: warning: {}", e);
    }

    /// Print an error to stderr and exit with code.
    ///
    /// The error printed is simply a message that is passed to this function
    /// along with the specified error code. Therefore, there is not much
    /// restriction to this call.
    pub fn exit(&self, e: String, code: i32) {
        eprintln!("listener: fatal: {}", e);
        process::exit(code);
    }
}
