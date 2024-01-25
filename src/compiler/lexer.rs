//! A custom SpRE lexical analyzer.
//!

use std::error::Error;
use std::fmt;

use super::listener::ErrorListener;

use self::stream::{CharStream, TokenStream};
use self::token::TokenKind::*;
use self::token::{Position, Token, TokenKind};

pub mod stream;
pub mod token;

pub struct Lexer {
    stream: CharStream,
    listener: Option<ErrorListener>,
    base: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new [`Lexer`].
    pub fn new(stream: CharStream) -> Self {
        Lexer {
            stream,
            listener: None,
            base: 0,
            current: 0,
            line: 1,
            column: 0,
        }
    }

    /// Attach an [`ErrorListener`] to the [`Lexer`].
    ///
    /// This attachment allows for better syntactical error reporting by the
    /// lexical analysis process based on the provided listener. If an error
    /// listener is not attached to the [`Lexer`], it panics.
    pub fn attach(mut self, listener: ErrorListener) -> Self {
        self.listener = Some(listener);
        self
    }

    /// Entrypoint function to lexically analyze a [`CharStream`].
    ///
    /// This method continuously attempts to tokenize the set of characters
    /// until an EOF is reached.
    ///
    /// In addition, an [`EndOfFile`] token is appended to the resulting token
    /// stream. Therefore, all returned [`TokenStream`]s will always have at
    /// least this token.
    pub fn lex(&mut self) -> TokenStream {
        let mut tokens = TokenStream::new();

        while !self.eof() {
            self.base = self.current;

            match self.next() {
                Ok(optional) => match optional {
                    Some(token) => tokens.push(token),
                    None => continue,
                },
                Err(_) => match &self.listener {
                    Some(listener) => {
                        listener.report(format!(
                            "lexer: ignoring unrecognized character: `{}`.",
                            self.stream.buffer[self.current - 1]
                        ));
                    }
                    None => panic!(),
                },
            }
        }

        // end token stream with EndOfFile
        tokens.push(Token::eof(Position(self.line, self.current - self.column)));
        tokens
    }

    /// Retrieve the next potential token from the [`CharStream`].
    ///
    /// A token is optionally returned. If a whitespace character is observed,
    /// [`None`] is returned.
    fn next(&mut self) -> Result<Option<Token>, Box<dyn Error>> {
        match self.advance() {
            '(' => Ok(self.tokenize(LeftParen)),
            ')' => Ok(self.tokenize(RightParen)),
            '{' => Ok(self.tokenize(LeftBrace)),
            '}' => Ok(self.tokenize(RightBrace)),
            '[' => Ok(self.tokenize(LeftBracket)),
            ']' => Ok(self.tokenize(RightBracket)),
            '<' => Ok(self.functionify(LeftChevron)),
            '>' => Ok(self.tokenize(RightChevron)),
            ',' => Ok(self.tokenize(Comma)),
            ':' => Ok(self.tokenize(Colon)),
            '*' => Ok(self.tokenize(Star)),
            '%' => Ok(self.tokenize(Percent)),
            '!' => Ok(self.tokenize(Not)),
            '&' => Ok(self.tokenize(And)),
            '|' => Ok(self.tokenize(Or)),
            '\n' => Ok(self.newline()),
            ' ' | '\r' | '\t' => Ok(self.skip(0)),
            '0'..='9' => Ok(self.numberify()),
            'a'..='z' | 'A'..='Z' | '_' => Ok(self.identifierify()),
            c => Err(Box::new(LexerError::from(format!(
                "unknown character `{}'",
                c
            )))),
        }
    }

    /// Build a specified token and capture the associated lexeme.
    fn tokenize(&self, kind: TokenKind) -> Option<Token> {
        let lexeme: String = self
            .stream
            .buffer
            .iter()
            .skip(self.base)
            .take(self.current - self.base)
            .collect();

        Some(Token::new(
            kind,
            Position(self.line, self.base - self.column),
            lexeme,
        ))
    }

    /// Advance the [`current`](Lexer::current), capturing the function.
    ///
    /// This method is used for identifying functions that are bounded with a
    /// left and right chevron (e.g., `<FUNCTION-NAME>`).
    fn functionify(&mut self, kind: TokenKind) -> Option<Token> {
        if let Some('a'..='z' | 'A'..='Z' | '_') = self.peek(0) {
            let name = self.identifierify().unwrap().lexeme;
            self.advance();

            self.tokenize(self.functionit(name).unwrap())
        } else {
            self.tokenize(kind)
        }
    }

    /// Map the function's name to an equivalent [`TokenKind`].
    ///
    /// The name of the function is case-sensitive.
    fn functionit(&self, name: String) -> Option<TokenKind> {
        match &name[1..] {
            "nonempty" => Some(NonEmpty),
            _ => match &self.listener {
                Some(listener) => {
                    listener.exit(format!("lexer: `{}` function not supported.", name), 1);
                    None
                }
                None => panic!(),
            },
        }
    }

    /// Advance the [`current`](Lexer::current), greedily consuming number characters.
    ///
    /// This method recognizes both [`Integer`]s and [`Real`]s based on the
    /// existence of a dot.
    fn numberify(&mut self) -> Option<Token> {
        while let Some(character) = self.peek(0) {
            if character.is_ascii_digit() {
                self.advance();
                continue;
            }

            break;
        }

        if let Some('.') = self.peek(0) {
            self.advance(); // consume Dot

            while let Some(character) = self.peek(0) {
                if character.is_ascii_digit() {
                    self.advance();
                    continue;
                }

                break;
            }

            self.tokenize(Real)
        } else {
            self.tokenize(Integer)
        }
    }

    /// Advance the [`current`](Lexer::current), greedily consuming identifier characters.
    fn identifierify(&mut self) -> Option<Token> {
        while let Some(character) = self.peek(0) {
            match character {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.advance();
                    continue;
                }
                _ => break,
            }
        }

        self.tokenize(Identifier)
    }

    /// Lookahead a number of characters into the character stream.
    ///
    /// If zero is provided, this will peek the current character due to the
    /// [`next`](Lexer::next) function incrementing the current.
    fn peek(&self, distance: usize) -> Option<char> {
        if self.current + distance >= self.stream.size {
            return None;
        }

        Some(self.stream.buffer[self.current + distance])
    }

    /// Get the current character and advance.
    fn advance(&mut self) -> char {
        self.current += 1;
        self.stream.buffer[self.current - 1]
    }

    /// Skip a number of characters.
    ///
    /// If zero is provided, this will skip the current character due to the
    /// [`next`](Lexer::next) function incrementing the current.
    fn skip(&mut self, distance: usize) -> Option<Token> {
        for _ in 0..distance {
            self.advance();
        }

        None
    }

    /// Increment the number of lines and skip.
    fn newline(&mut self) -> Option<Token> {
        self.line += 1;
        self.column = self.current;

        self.skip(0)
    }

    fn eof(&self) -> bool {
        self.current >= self.stream.size
    }
}

#[derive(Debug, Clone)]
struct LexerError {
    msg: String,
}

impl From<&str> for LexerError {
    fn from(msg: &str) -> Self {
        LexerError {
            msg: msg.to_string(),
        }
    }
}

impl From<String> for LexerError {
    fn from(msg: String) -> Self {
        LexerError { msg }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lexer: {}", self.msg)
    }
}

impl Error for LexerError {}

#[cfg(test)]
mod tests {
    use super::{stream::CharStream, token::Position, token::Token, token::TokenKind, Lexer};

    #[test]
    fn lex_left_parens() {
        let mut lexer = Lexer::new(CharStream::from("((("));

        lexer.base = lexer.current;
        assert_eq!(
            Token {
                kind: TokenKind::LeftParen,
                position: Position(1, 0),
                lexeme: String::from("(")
            },
            lexer.next().ok().unwrap().unwrap()
        );

        lexer.base = lexer.current;
        assert_eq!(
            Token {
                kind: TokenKind::LeftParen,
                position: Position(1, 1),
                lexeme: String::from("(")
            },
            lexer.next().ok().unwrap().unwrap()
        );

        lexer.base = lexer.current;
        assert_eq!(
            Token {
                kind: TokenKind::LeftParen,
                position: Position(1, 2),
                lexeme: String::from("(")
            },
            lexer.next().ok().unwrap().unwrap()
        );
    }
}
