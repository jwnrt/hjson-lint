use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

/// Kinds of token in the Hjson grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Bool,
    LineComment,
    BlockComment,
    HashComment,
    Null,
    Integer,
    Float,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    TextSingle,
    TextDouble,
    TextMulti,
    TextUnquoted,
    NewLine,
    Whitespace,
}

impl TokenKind {
    /// Create a [`Token`] with this `kind` and the given `len`.
    pub fn with_len(self, len: usize) -> Token {
        Token { kind: self, len }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Eof => "EOF",
            Self::Bool => "Boolean",
            Self::LineComment => "line comment",
            Self::BlockComment => "block comment",
            Self::HashComment => "hash comment",
            Self::Null => "null",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::LBrace => "{",
            Self::RBrace => "}",
            Self::LBracket => "[",
            Self::RBracket => "]",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::TextSingle => "single-quoted string",
            Self::TextDouble => "double-quoted string",
            Self::TextMulti => "multi-line string",
            Self::TextUnquoted => "unquoted string",
            Self::NewLine => "newline",
            Self::Whitespace => "whitespace",
        };
        f.write_str(name)
    }
}
