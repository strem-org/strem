//! Symbolic Abstract Syntax Tree (SAST) representation.
//!
//! This Intermediate Representation (IR) of the SpRE is a necessary step to
//! perform matching with the underlying library.

use crate::compiler::ir::{ast::SpatialFormula, Node};

/// A symbolically-linked spatial formula.
///
/// This maps a [`SpatialFormula`] to a unique symbol that is used when
/// performing matching.
pub struct SymbolicFormula {
    pub symbol: char,
    pub formula: SpatialFormula,
}

impl SymbolicFormula {
    pub fn new(symbol: char, formula: SpatialFormula) -> Self {
        Self { symbol, formula }
    }
}

/// The symbolically-represented AST.
///
/// Within this AST, each internal node is a RE-based operation (e.g.,
/// alternation, concatenation, etc); and each operand is a [`SymbolicFormula`].
pub struct SymbolicAbstractSyntaxTree {
    pub root: Option<Node<SymbolicFormula>>,
}

impl SymbolicAbstractSyntaxTree {
    pub fn new(root: Option<Node<SymbolicFormula>>) -> Self {
        Self { root }
    }

    /// From the symbolic-AST, return the set of spatial formulas.
    pub fn fmap(&self) -> Vec<&SymbolicFormula> {
        if let Some(root) = &self.root {
            return SymbolicAbstractSyntaxTree::fmapit(root);
        }

        Vec::new()
    }

    /// The recursive helper function to generate the list of spatial formulas.
    ///
    /// The result is a list of [`SymbolicFormula`] found on the symbolic-AST.
    /// Since the AST operands are [`SymbolicFormula`], for each operand, the
    /// tree is returned.
    fn fmapit(node: &Node<SymbolicFormula>) -> Vec<&SymbolicFormula> {
        match node {
            Node::Operand(sformula) => vec![sformula],
            Node::UnaryExpr { child, .. } => SymbolicAbstractSyntaxTree::fmapit(child),
            Node::BinaryExpr { left, right, .. } => {
                let mut formulas = SymbolicAbstractSyntaxTree::fmapit(left);
                formulas.extend(SymbolicAbstractSyntaxTree::fmapit(right));

                formulas
            }
        }
    }
}
