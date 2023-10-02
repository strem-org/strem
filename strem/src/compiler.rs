//! The compiler framework for SpREs.
//!

pub mod analyzer;
pub mod ir;
pub mod lexer;
pub mod listener;
pub mod parser;

use std::error;

use crate::{
    compiler::{
        lexer::{stream::CharStream, Lexer},
        listener::ErrorListener,
        parser::Parser,
    },
    symbolizer::{ast::SymbolicAbstractSyntaxTree as AST, Symbolizer},
};

const ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Default)]
pub struct Compiler {}

impl Compiler {
    /// Create a new [`Compiler`].
    pub fn new() -> Self {
        Compiler {}
    }

    /// Compile a Spatial Regular Expression (SpRE) into an Abstract Syntax
    /// Tree (AST).
    ///
    /// To compile, a string is expected. Therefore, any file
    /// handling/interfacing must be done beforehand and converted appropriately.
    pub fn compile(&self, source: &str) -> Result<AST, Box<dyn error::Error>> {
        let stream = CharStream::from(source);

        let mut lexer = Lexer::new(stream).attach(ErrorListener::new());
        let stream = lexer.lex();

        let mut parser = Parser::new(stream).attach(ErrorListener::new());
        let ast = parser.parse();

        // TODO: Implement semantic analysis on the resulting AST
        //
        // let semantic = SemanticAnalyzer::new();
        // let ast = semantic.analyze(ast);

        let mut symbolizer = Symbolizer::new(&self::ALPHABET);
        let ast = symbolizer.symbolize(ast)?;

        Ok(ast)
    }
}
