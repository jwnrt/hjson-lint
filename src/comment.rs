use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comment {
    Line,
    Block,
    Hash,
}

impl Parse for Comment {
    fn parse(input: &str) -> Option<Token> {
        todo!()
    }
}
