//! Semantic analysis framework for SpREs.
//!

use super::super::ir::ast::AbstractSyntaxTree;

#[derive(Default)]
pub struct SemanticAnalyzer {}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {}
    }

    #[allow(unused_variables)]
    pub fn analyze(&self, tree: &AbstractSyntaxTree) {
        std::unimplemented!()
    }
}
