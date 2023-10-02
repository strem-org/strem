//! Semantic analysis framework for SpREs.
//!

use crate::compiler::ir::ast::AbstractSyntaxTree;

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
