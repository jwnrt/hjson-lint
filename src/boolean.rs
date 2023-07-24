use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Boolean;

impl Parse for Boolean {
    fn parse(input: &str) -> Option<Token> {
        if input.starts_with("true") {
            Some(Token::new(Boolean, 4))
        } else if input.starts_with("false") {
            Some(Token::new(Boolean, 5))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boolean() {
        assert_eq!(Boolean::parse("true"), Some(Token::new(Boolean, 4)));
        assert_eq!(Boolean::parse("false"), Some(Token::new(Boolean, 5)));
        assert_eq!(Boolean::parse("true "), Some(Token::new(Boolean, 4)));
        assert_eq!(Boolean::parse(" true"), None);
        assert_eq!(Boolean::parse(""), None);
    }
}
