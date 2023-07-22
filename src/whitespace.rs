use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Whitespace {
    NewLine,
    Other,
}

impl Parse for Whitespace {
    fn parse(input: &str) -> Option<Token> {
        todo!()
    }
}
