use crate::{
    compiler::ir::{ast::SpatialFormula, Node, Operator, S4OperatorKind, SpatialOperatorKind},
    datastream::reader::detection::{
        annotation::{Annotation, BoundingBox},
        Detection,
    },
};

/// A monitor for evaluating S4u formulas.
#[derive(Default)]
pub struct Monitor {}

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, detection: &Detection, formula: &SpatialFormula) -> Vec<Annotation> {
        match formula {
            Node::Operand(label) => {
                // Retrieve an annotation with the same class category as
                // specified by the label.
                if let Some(annotation) = self.lookup(label, detection) {
                    return vec![annotation];
                }

                Vec::new()
            }
            Node::UnaryExpr { op, .. } => match op {
                Operator::SpatialOperator(op) => match op {
                    SpatialOperatorKind::S4Operator(op) => match op {
                        S4OperatorKind::Complement => {
                            todo!()
                        }
                        _ => panic!("monitor: s4: unrecognized unary operator"),
                    },
                    _ => panic!("monitor: s4: unrecognized unary operator"),
                },
                _ => panic!("monitor: s4: unrecognized unary operator"),
            },
            Node::BinaryExpr { op, left, right } => {
                let left = self.evaluate(detection, left);
                let right = self.evaluate(detection, right);

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
                                        if Self::intersection(&l.bbox, &r.bbox) {
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
                                left.into_iter().chain(right.into_iter()).collect()
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

    /// Compute the intersection of two bounding boxes.
    ///
    /// If no intersection exists, then [`None`] is returned which is
    /// semantically equivalent to the empty set.
    fn intersection(a: &BoundingBox, b: &BoundingBox) -> bool {
        let amin = a.min();
        let amax = a.max();
        let bmin = b.min();
        let bmax = b.max();

        // check if overlap exists
        if amin.0 < bmax.0 && bmin.0 < amax.0 && amin.1 < bmax.1 && bmin.1 < amax.1 {
            let _min = (
                std::cmp::max(amin.0 as i64, bmin.0 as i64) as f64,
                std::cmp::max(amin.1 as i64, bmin.1 as i64) as f64,
            );

            let _max = (
                std::cmp::min(amax.0 as i64, bmax.0 as i64) as f64,
                std::cmp::min(amax.1 as i64, bmax.1 as i64) as f64,
            );

            true
        } else {
            false
        }
    }
}
