//! Compiler framework input data streams.
//!

use super::token::Token;

#[derive(Debug)]
pub struct CharStream {
    pub buffer: Vec<char>,
    pub size: usize,
}

impl From<&str> for CharStream {
    fn from(source: &str) -> Self {
        CharStream {
            buffer: source.chars().collect(),
            size: source.len(),
        }
    }
}

#[derive(Debug, Default)]
pub struct TokenStream {
    pub buffer: Vec<Token>,
    pub size: usize,
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream {
            buffer: Vec::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, token: Token) {
        self.buffer.push(token);
        self.size += 1;
    }
}
