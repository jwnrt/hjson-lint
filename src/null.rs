use crate::TokenKind;

use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Null;

impl From<Null> for TokenKind {
    fn from(_: Null) -> Self {
        TokenKind::Null
    }
}

impl Parse for Null {
    fn parse(input: &str) -> Option<Token> {
        match input.starts_with("null") {
            true => Some(Token::new(Null, 4)),
            false => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn null() {
        assert_eq!(Null::parse("null"), Some(Token::new(Null, 4)));
        assert_eq!(Null::parse("null "), Some(Token::new(Null, 4)));
        assert_eq!(Null::parse(" null"), None);
        assert_eq!(Null::parse(""), None);
    }
}
