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
    Boolean(Boolean),
    Comment(Comment),
    Key(Key),
    Null(Null),
    Number(Number),
    Symbol(Symbol),
    Text(Text),
    Whitespace(Whitespace),
}

impl From<Boolean> for TokenKind {
    fn from(value: Boolean) -> Self {
        Self::Boolean(value)
    }
}

impl From<Comment> for TokenKind {
    fn from(value: Comment) -> Self {
        Self::Comment(value)
    }
}

impl From<Key> for TokenKind {
    fn from(value: Key) -> Self {
        Self::Key(value)
    }
}

impl From<Null> for TokenKind {
    fn from(value: Null) -> Self {
        Self::Null(value)
    }
}

impl From<Number> for TokenKind {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<Symbol> for TokenKind {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

impl From<Text> for TokenKind {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl From<Whitespace> for TokenKind {
    fn from(value: Whitespace) -> Self {
        Self::Whitespace(value)
    }
}
