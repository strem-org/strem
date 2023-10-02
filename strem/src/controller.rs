//! The matching frameworks controller.
//!
//! This module is responsible for managing and controlling the behavior of the
//! matching framework.

use crate::config::Configuration;

/// The main driver to perform matching.
///
/// This includes processing datastreams, monitoring, and matching. The main
/// influence on the controller is from the [`Configuration`] scheme.
#[allow(dead_code)]
pub struct Controller<'a> {
    config: &'a Configuration<'a>,
}

impl<'a> Controller<'a> {
    /// Create new [`Controller`] with associated [`Configuration`].
    pub fn new(config: &'a Configuration) -> Self {
        Self { config }
    }

    /// Run the main loop of the [`Controller`].
    ///
    /// This is the entrypoint to start the matching framework procedure.
    pub fn run(&self) {}
}
