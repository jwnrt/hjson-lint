use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Text {
    Single,
    Double,
    Multi,
    Unquoted,
}

impl Parse for Text {
    fn parse(input: &str) -> Option<Token> {
        todo!()
    }
}
