use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Null;

impl Parse for Null {
    fn parse(input: &str) -> Option<Token> {
        todo!()
    }
}
