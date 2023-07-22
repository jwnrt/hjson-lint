use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Number {
    Float,
    Integer,
}

impl Parse for Number {
    fn parse(input: &str) -> Option<Token> {
        todo!()
    }
}
