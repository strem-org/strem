//! A traditional Regular Expression (RE) module.
//!
//! This module is primarily intended to connect the pattern matching approach
//! with pre-existing libraries that perform pattern matching over traditional
//! RE patterns.
//!
//! This module is responsible for the following:
//! 1. Building symbolically equivalent REs from SpREs.
//! 2. Generating the underlying DFAs to perform pattern matching.

use crate::{
    compiler::ir::{Node, Operator, RangeKind, RegexOperatorKind},
    symbolizer::ast::{SymbolicAbstractSyntaxTree, SymbolicFormula},
};

use super::automata::{dfa::DeterministicFiniteAutomata, Builder};

/// A Regular Expression (RE) for finding SpRE matches.
///
/// This RE contains a single reverse DFA that matches the reversed input. This
/// choice is intentional as the input is assumed to be incrementally provided.
/// Therefore, the end position is always as it is the most recent input.
/// However, the start position must be found by executing the reverse DFA,
/// accordingly.
pub struct RegularExpression {
    pub pattern: String,
    pub reverse: DeterministicFiniteAutomata,
}

impl RegularExpression {
    pub fn step(&mut self, bytes: &[u8]) {
        self.reverse.step(bytes);
    }

    pub fn isdead(&self) -> bool {
        self.reverse.isdead()
    }

    pub fn ismatch(&mut self) -> bool {
        self.reverse.ismatch()
    }

    pub fn eoi(&self) -> bool {
        self.reverse.eoi()
    }

    pub fn reset(&mut self) {
        self.reverse.reset();
    }
}

impl From<&SymbolicAbstractSyntaxTree> for RegularExpression {
    /// From a [`SymbolicAbstractSyntaxTree`], produce a [`RegularExpression`].
    ///
    /// The [`RegularExpression`] is built by first constructing the symbolic RE
    /// pattern followed by the corresponding DFA to match the RE.
    fn from(ast: &SymbolicAbstractSyntaxTree) -> Self {
        let pattern = self::regexify(ast);
        let reverse = Builder::new().reverse().build(&pattern).unwrap();

        RegularExpression { pattern, reverse }
    }
}

/// Construct a Regular Expression (RE) pattern from a [`AbstractSyntaxTree`].
///
/// This traverses the outer components of a SpRE related solely to the RE-based
/// patterns.
fn regexify(ast: &SymbolicAbstractSyntaxTree) -> String {
    if let Some(root) = &ast.root {
        return self::regexit(root);
    }

    String::new()
}

/// Recursively construct an RE.
///
/// This is the helper function that performs walks the [`AbstractSyntaxTree`]
/// to build the appropriate pattern.
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
