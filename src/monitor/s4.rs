use std::collections::HashMap;

use crate::compiler::ir::ast::SpatialFormula;
use crate::compiler::ir::{Node, Operator, S4OperatorKind, SpatialOperatorKind};
use crate::datastream::frame::sample::detections::{Annotation, BoundingBox, Point};

/// A monitor for evaluating S4 formulas.
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
    ) -> Vec<Annotation> {
        match formula {
            Node::Operand(label) => {
                // Retrieve an annotation with the same class category as
                // specified by the label.
                if let Some(annotation) = detections.get(label) {
                    return annotation.clone();
                }

                Vec::new()
            }
            Node::UnaryExpr { op, .. } => match op {
                Operator::SpatialOperator(SpatialOperatorKind::S4Operator(
                    S4OperatorKind::Complement,
                )) => {
                    todo!()
                }
                _ => panic!("monitor: s4: unrecognized unary operator"),
            },
            Node::BinaryExpr { op, left, right } => {
                let left = Monitor::evaluate(detections, left);
                let right = Monitor::evaluate(detections, right);

                match op {
                    Operator::SpatialOperator(op) => match op {
                        SpatialOperatorKind::S4Operator(op) => match op {
                            S4OperatorKind::Intersection => {
                                // If either left or right is empty, then one
                                // side is not satisfied. Therefore, the
                                // resulting formula is not satisifed, entirely.
                                if left.is_empty() || right.is_empty() {
                                    return Vec::new();
                                }

                                let mut intersections = Vec::new();

                                for l in left.iter() {
                                    for r in right.iter() {
                                        if let Some(_bbox) = Self::intersection(&l.bbox, &r.bbox) {
                                            intersections.push(l.clone());
                                            intersections.push(r.clone());
                                        }
                                    }
                                }

                                intersections
                            }
                            S4OperatorKind::Union => {
                                // We don't care which one satisfied---just as
                                // long as left or right is valid. Therefore, we
                                // append all solutions.
                                left.into_iter().chain(right).collect()
                            }
                            _ => panic!("monitor: s4: unknown binary operator"),
                        },
                        _ => panic!("monitor: unknown binary operator {:#?}", op),
                    },
                    _ => panic!("monitor: unknown binary operator {:#?}", op),
                }
            }
        }
    }

    /// Compute the intersection of two bounding boxes.
    ///
    /// If no intersection exists, then [`None`] is returned which is
    /// semantically equivalent to the empty set.
    fn intersection(a: &BoundingBox, b: &BoundingBox) -> Option<BoundingBox> {
        // check if overlap exists
        if a.min.x < b.max.x && b.min.x < a.max.x && a.min.y < b.max.y && b.min.y < a.max.y {
            let min = Point::new(
                std::cmp::max(a.min.x as i64, b.min.x as i64) as f64,
                std::cmp::max(a.min.y as i64, b.min.y as i64) as f64,
            );

            let max = Point::new(
                std::cmp::min(a.max.x as i64, b.max.x as i64) as f64,
                std::cmp::min(a.max.y as i64, b.max.y as i64) as f64,
            );

            return Some(BoundingBox::new(min, max));
        }

        None
    }
}
