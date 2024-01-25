//! The matching framework for SpREs.
//!

use std::error::Error;

use crate::compiler::ir::{Node, Operator, RangeKind, RegexOperatorKind};
use crate::datastream::frame::Frame;
use crate::symbolizer::ast::{SymbolicAbstractSyntaxTree, SymbolicFormula};

pub mod automata;
pub mod offline;
pub mod online;

/// A trait for which all matchers must implement.
///
/// This is defined to provide a ubiquitous interface for all matchers to adhere
/// to for simplicity of switching (e.g., facade pattern).
pub trait Matching {
    /// Find a possible leftmost [`Match`] from the set of [`Frame`].
    fn leftmost(&self, frames: &[Frame]) -> Result<Option<Match>, Box<dyn Error>>;
}

/// A range of valid indices.
///
/// It should be noted that `start` is inclusive (closed) while `end` is
/// exclusive (open); so a [`Match`] takes the form: [start, end). This is also
/// referred to as a half-open interval.
#[derive(Debug)]
pub struct Match {
    pub start: usize,
    pub end: usize,
}

impl Match {
    /// Create a new complete [`Match`] with start and end indices.
    pub fn new(start: usize, end: usize) -> Self {
        Match { start, end }
    }
}

/// Construct a Regular Expression (RE) pattern from a [`SymbolicAbstractSyntaxTree`].
///
/// This traverses the outer components of a SpRE related solely to the RE-based
/// patterns and symbols.
pub fn regexify(ast: &SymbolicAbstractSyntaxTree) -> String {
    if let Some(root) = &ast.root {
        return self::regexit(root);
    }

    String::new()
}

/// Recursively construct an RE.
///
/// This is the helper function that walks the root [`Node`] of a
/// [`SymbolicAbstractSyntaxTree`] to build the appropriate pattern.
fn regexit(node: &Node<SymbolicFormula>) -> String {
    match node {
        Node::Operand(formula) => String::from(formula.symbol),
        Node::UnaryExpr { op, child } => {
            let child = self::regexit(child);

            match op {
                Operator::RegexOperator(kind) => match kind {
                    RegexOperatorKind::KleeneStar => format!("({}*)", child),
                    RegexOperatorKind::Range(kind) => match kind {
                        RangeKind::Exactly(size) => format!("({}{{{}}})", child, size),
                        RangeKind::AtLeast(min) => format!("({}{{{},}})", child, min),
                        RangeKind::Between(min, max) => format!("({}{{{},{}}})", child, min, max),
                    },
                    _ => String::new(),
                },
                _ => String::new(),
            }
        }
        Node::BinaryExpr { op, left, right } => {
            let left = self::regexit(left);
            let right = self::regexit(right);

            match op {
                Operator::RegexOperator(kind) => match kind {
                    RegexOperatorKind::Concatenation => format!("({}{})", left, right),
                    RegexOperatorKind::Alternation => format!("({}|{})", left, right),
                    _ => String::new(),
                },
                _ => String::new(),
            }
        }
    }
}
