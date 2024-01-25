//! Abstract Syntax Tree (AST) representation.
//!

use super::super::ir::Node;

pub type SpatialFormula = Node<String>;

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub root: Option<Node<SpatialFormula>>,
}

impl AbstractSyntaxTree {
    pub fn new(root: Option<Node<SpatialFormula>>) -> Self {
        Self { root }
    }
}
