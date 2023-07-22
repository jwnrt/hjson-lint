use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Single,
    Double,
    Unquoted,
}

impl Parse for Key {
    fn parse(input: &str) -> Option<Token> {
        todo!();
    }
}
