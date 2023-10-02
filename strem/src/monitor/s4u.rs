use crate::{
    compiler::ir::{
        ast::SpatialFormula, FolOperatorKind, Node, Operator, S4uOperatorKind, SpatialOperatorKind,
    },
    datastream::reader::detection::{annotation::Annotation, Detection},
};

use super::s4;

/// A monitor for evaluating S4u formulas.
#[derive(Default)]
pub struct Monitor {}

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, detection: &Detection, formula: &SpatialFormula) -> bool {
        match formula {
            Node::Operand(label) => {
                // Retrieve an annotation with the same class category as
                // specified by the label.
                if let Some(..) = self.lookup(label, detection) {
                    return true;
                }

                false
            }
            Node::UnaryExpr { op, child } => match op {
                Operator::SpatialOperator(op) => match op {
                    SpatialOperatorKind::S4uOperator(op) => match op {
                        S4uOperatorKind::NonEmpty => {
                            !s4::Monitor::new().evaluate(detection, child).is_empty()
                        }
                    },
                    SpatialOperatorKind::FolOperator(op) => match op {
                        FolOperatorKind::Negation => {
                            let child = self.evaluate(detection, child);
                            !child
                        }
                        _ => panic!("monitor: s4u: unrecognized unary FOL operator"),
                    },
                    _ => panic!("monitor: s4u: unrecognized unary operator"),
                },
                _ => panic!("monitor: s4u: unrecognized unary operator"),
            },
            Node::BinaryExpr { op, left, right } => {
                let left = self.evaluate(detection, left);
                let right = self.evaluate(detection, right);

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

    /// Check whether a class annotation exists.
    ///
    /// The existence of the category does not guarantee that a `true` is
    /// returned. It also must be true that at least one annotation must
    /// exist.
    fn lookup(&self, class: &String, detection: &Detection) -> Option<Annotation> {
        if let Some(category) = detection.categories.get(class) {
            if let Some(annotations) = detection.annotations.get(&category.id) {
                if let Some(annotation) = annotations.get(0) {
                    return Some(annotation.clone());
                }
            }
        }

        None
    }
}
