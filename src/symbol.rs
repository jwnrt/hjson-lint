use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
}

impl Parse for Symbol {
    fn parse(input: &str) -> Option<Token> {
        todo!()
    }
}
