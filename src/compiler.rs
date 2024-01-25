//! The compiler framework for SpREs.
//!

use std::error::Error;

use crate::symbolizer::ast::SymbolicAbstractSyntaxTree;
use crate::symbolizer::Symbolizer;

use self::lexer::stream::CharStream;
use self::lexer::Lexer;
use self::listener::ErrorListener;
use self::parser::Parser;

pub mod analyzer;
pub mod ir;
pub mod lexer;
pub mod listener;
pub mod parser;

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
    pub fn compile(&self, source: &str) -> Result<SymbolicAbstractSyntaxTree, Box<dyn Error>> {
        let stream = CharStream::from(source);

        let mut lexer = Lexer::new(stream).attach(ErrorListener::new());
        let stream = lexer.lex();

        let mut parser = Parser::new(stream).attach(ErrorListener::new());
        let ast = parser.parse();

        let mut symbolizer = Symbolizer::new(&self::ALPHABET);
        let ast = symbolizer.symbolize(ast)?;

        Ok(ast)
    }
}
