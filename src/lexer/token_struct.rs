use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(tag = "type")]
pub enum TokenKind {
    // numbers
    Number(f64),

    // operators
    Asterisk,    // *
    Equals,      // =
    Plus,        // +
    Slash,       // /
    LessThan,    // <
    GreaterThan, // >
    Minus,       // -
    Colon,       // :

    // Words
    Identifier(String),
    QuotedString(String),

    // WS
    Indentation(usize),

    // Keywords
    WidgetKW,

    // Misc
    At,          // @
    Dot,         // .
    CloseParen,  // )
    CloseSquare, // ]
    OpenParen,   // (
    OpenSquare,  // [
    Semicolon,   // ;
    End,         // EOF
}

impl From<String> for TokenKind {
    fn from(other: String) -> TokenKind {
        TokenKind::Identifier(other)
    }
}

impl<'a> From<&'a str> for TokenKind {
    fn from(other: &'a str) -> TokenKind {
        TokenKind::Identifier(other.to_string())
    }
}

impl From<f64> for TokenKind {
    fn from(other: f64) -> TokenKind {
        TokenKind::Number(other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize, line: usize) -> Self {
        Self {
            kind,
            start,
            end,
            line,
        }
    }
}
