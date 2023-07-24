mod boolean;
mod comment;
mod iter;
mod key;
mod null;
mod number;
mod symbol;
mod text;
mod whitespace;

pub use boolean::Boolean;
pub use comment::Comment;
pub use iter::Tokens;
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
pub struct Token {
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
