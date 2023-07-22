use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Boolean;

impl Parse for Boolean {
    fn parse(input: &str) -> Option<Token> {
        todo!()
    }
}
