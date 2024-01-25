//! This module provides the necessary monitors to evaluate spatial formulas.
//!
//! Currently, the implemented monitors include evaluation of S4/S4u topological
//! formulas interpreted over frames.

use crate::compiler::ir::ast::SpatialFormula;
use crate::datastream::frame::sample::Sample;
use crate::datastream::frame::Frame;

pub mod s4;
pub mod s4u;

/// The main monitor.
///
/// This is a entrypoint for monitoring spatial formulas found within SpREs. This
/// interface is also responsible for managing evaluating these formulas against
/// different sample types.
///
/// For example, point clouds, object detections, etc.
#[derive(Default)]
pub struct Monitor {}

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }

    /// The main interface to evaluating a frame sample against a spatial formula.
    ///
    /// This considers all possible sample types.
    pub fn evaluate(&self, frame: &Frame, formula: &SpatialFormula) -> bool {
        for sample in frame.samples.iter() {
            match sample {
                Sample::ObjectDetection(record) => {
                    if s4u::Monitor::evaluate(&record.annotations, formula) {
                        return true;
                    }
                }
            };
        }

        false
    }
}
