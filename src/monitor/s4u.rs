use std::collections::HashMap;

use crate::compiler::ir::ast::SpatialFormula;
use crate::compiler::ir::{FolOperatorKind, Node, Operator, S4uOperatorKind, SpatialOperatorKind};
use crate::datastream::frame::sample::detections::Annotation;

use super::s4;

/// A monitor for evaluating S4u formulas.
///
/// This monitor evaluates against a series of object detection obtained from the
/// perception stream.
#[derive(Default)]
pub struct Monitor {}

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }

    /// Evaluate formula satisfaction against set of annotations.
    ///
    /// This returns is a boolean result. If true, the formula is satisifed;
    /// else, if false, then it is not satisfied.
    pub fn evaluate(
        detections: &HashMap<String, Vec<Annotation>>,
        formula: &SpatialFormula,
    ) -> bool {
        match formula {
            Node::Operand(label) => {
                if detections.get(label).is_some() {
                    return true;
                }

                false
            }
            Node::UnaryExpr { op, child } => match op {
                Operator::SpatialOperator(op) => match op {
                    SpatialOperatorKind::S4uOperator(op) => match op {
                        S4uOperatorKind::NonEmpty => {
                            !s4::Monitor::evaluate(detections, child).is_empty()
                        }
                    },
                    SpatialOperatorKind::FolOperator(op) => match op {
                        FolOperatorKind::Negation => {
                            let res = Monitor::evaluate(detections, child);
                            !res
                        }
                        _ => panic!("monitor: s4u: unrecognized unary FOL operator"),
                    },
                    _ => panic!("monitor: s4u: unrecognized unary operator"),
                },
                _ => panic!("monitor: s4u: unrecognized unary operator"),
            },
            Node::BinaryExpr { op, left, right } => {
                let left = Monitor::evaluate(detections, left);
                let right = Monitor::evaluate(detections, right);

                match op {
                    Operator::SpatialOperator(kind) => match kind {
                        SpatialOperatorKind::FolOperator(kind) => match kind {
                            FolOperatorKind::Conjunction => left && right,
                            FolOperatorKind::Disjunction => left || right,
                            _ => panic!("monitor: unkown FOL operator {:#?}", kind),
                        },
                        _ => panic!("monitor: unknown binary operator {:#?}", kind),
                    },
                    _ => panic!("monitor: unknown binary operator {:#?}", op),
                }
            }
        }
    }
}
