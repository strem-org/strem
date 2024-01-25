//! Intermediate module for symbolic structures.
//!
//! This module is necessary to effectively match against a Spatial Regular
//! Expression (SpRE).
//!
//! Generally, it provides the mechanisms and interfaces to map a each unique
//! spatial-based formula to be evaluate to a unique symbol.

use std::error::Error;
use std::fmt;

use crate::compiler::ir::ast::{AbstractSyntaxTree, SpatialFormula};
use crate::compiler::ir::Node;

use self::ast::{SymbolicAbstractSyntaxTree, SymbolicFormula};

pub mod ast;

#[derive(Default)]
pub struct Symbolizer<'a> {
    current: usize,
    alphabet: &'a [char],
}

impl<'a> Symbolizer<'a> {
    /// Create a new [`Symbolizer`] with provided alphabet.
    pub fn new(alphabet: &'a [char]) -> Self {
        let current = 0;
        Self { current, alphabet }
    }

    /// Construct a [`SymbolicAbstractSyntaxTree`] from an [`AbstractSyntaxTree`].
    ///
    /// This step is used for the matcher that requires symbols to execute its
    /// underlying matching mechanisms.
    pub fn symbolize(
        &mut self,
        ast: AbstractSyntaxTree,
    ) -> Result<SymbolicAbstractSyntaxTree, Box<dyn Error>> {
        if let Some(root) = ast.root {
            return Ok(SymbolicAbstractSyntaxTree::new(Some(
                self.symbolizeit(root)?,
            )));
        }

        Ok(SymbolicAbstractSyntaxTree::new(None))
    }

    /// Recursively build the Symbolic Abstract Syntax Tree.
    ///
    /// The main procedure done here is to take each root node of the spatial
    /// formulas and wrap the root node with a uniquely mapped symbol.
    fn symbolizeit(
        &mut self,
        node: Node<SpatialFormula>,
    ) -> Result<Node<SymbolicFormula>, Box<dyn Error>> {
        match node {
            Node::Operand(formula) => {
                let symbol = self.advance()?;
                Ok(Node::Operand(SymbolicFormula::new(symbol, formula)))
            }
            Node::UnaryExpr { op, child } => {
                let child = self.symbolizeit(*child)?;
                Ok(Node::unary(op, child))
            }
            Node::BinaryExpr { op, left, right } => {
                let left = self.symbolizeit(*left)?;
                let right = self.symbolizeit(*right)?;

                Ok(Node::binary(op, left, right))
            }
        }
    }

    /// Retrieve the next unique symbol in the alphabet.
    ///
    /// This procedure will raise an error if an insufficient number of symbols
    /// are present for the number of spatial formulas written.
    fn advance(&mut self) -> Result<char, Box<dyn Error>> {
        if let Some(symbol) = self.alphabet.get(self.current) {
            self.current += 1;
            return Ok(*symbol);
        }

        Err(Box::new(SymbolizerError::from(format!(
            "insufficient symbols ({}) for formulas ({})",
            self.alphabet.len(),
            self.current
        ))))
    }
}

#[derive(Debug, Clone)]
struct SymbolizerError {
    msg: String,
}

impl From<&str> for SymbolizerError {
    fn from(msg: &str) -> Self {
        SymbolizerError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for SymbolizerError {
    fn from(msg: String) -> Self {
        SymbolizerError { msg }
    }
}

impl fmt::Display for SymbolizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "symbolizer: {}", self.msg)
    }
}

impl Error for SymbolizerError {}
