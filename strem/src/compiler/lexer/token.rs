//! Lexical unit information.
//!

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftChevron,
    RightChevron,
    Comma,
    Colon,
    Star,
    Percent,
    Not,
    And,
    Or,
    EndOfFile,
    Integer,
    Real,
    Identifier,
    NonEmpty,
}

/// Locational information used in a [`Token`].
///
/// This includes the row and the column number where the token begins.
/// **Note**: The beginning of the source is located at (1, 0).
#[derive(Clone, Debug, PartialEq)]
pub struct Position(pub usize, pub usize);

/// A lexical unit produced during tokenization by the lexical analyzer.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, position: Position, lexeme: String) -> Self {
        Token {
            kind,
            position,
            lexeme,
        }
    }

    pub fn eof(position: Position) -> Self {
        Token {
            kind: TokenKind::EndOfFile,
            position,
            lexeme: String::new(),
        }
    }
}
