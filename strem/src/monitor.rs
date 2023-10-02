//! This module provides the necessary monitors to evaluate spatial formulas.
//!
//! Currently, the implemented monitors include evaluation of S4/S4u topological
//! formulas interpreted over frames.

use crate::{
    compiler::ir::ast::SpatialFormula,
    datastream::{
        frame::Frame,
        reader::{detection::Detection, Sample},
    },
};

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

    /// The main interface to evaluating a frame against a spatial formula.
    ///
    /// This considers all possible sample types.
    pub fn evaluate(&self, frame: &Frame, formula: &SpatialFormula) -> bool {
        for sample in frame.samples.iter() {
            match sample {
                Sample::ObjectDetection(d) => return self.evaldet(d, formula),
            };
        }

        false
    }

    /// Evaluate a detection against a formula.
    fn evaldet(&self, detection: &Detection, formula: &SpatialFormula) -> bool {
        s4u::Monitor::new().evaluate(detection, formula)
    }
}
