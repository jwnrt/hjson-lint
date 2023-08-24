mod boolean;
mod comment;
mod iter;
mod key;
mod null;
mod number;
mod symbol;
mod text;
mod whitespace;

use std::fmt::{self, Display};

pub use boolean::Boolean;
pub use comment::Comment;
pub use iter::{Cursor, Span, Tokens};
pub use key::Key;
pub use null::Null;
pub use number::Number;
pub use symbol::Symbol;
pub use text::Text;
pub use whitespace::Whitespace;

trait Parse: Sized {
    fn parse(input: &str) -> Option<Token>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new<T: Into<TokenKind>>(kind: T, len: usize) -> Self {
        Self {
            kind: kind.into(),
            len,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    Boolean,
    LineComment,
    BlockComment,
    HashComment,
    Null,
    Integer,
    Float,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    TextSingle,
    TextDouble,
    TextMulti,
    TextUnquoted,
    NewLine,
    Whitespace,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Eof => "EOF",
            Self::Boolean => "Boolean",
            Self::LineComment => "line comment",
            Self::BlockComment => "block comment",
            Self::HashComment => "hash comment",
            Self::Null => "null",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::OpenBrace => "{",
            Self::CloseBrace => "}",
            Self::OpenBracket => "[",
            Self::CloseBracket => "]",
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
