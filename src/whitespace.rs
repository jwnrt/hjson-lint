use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Whitespace {
    NewLine,
    Other,
}

impl Parse for Whitespace {
    fn parse(input: &str) -> Option<Token> {
        if input.starts_with('\n') {
            return Some(Token::new(Whitespace::NewLine, 1));
        }

        let non_whitespace = input
            .find(|c: char| c == '\n' || !c.is_whitespace())
            .unwrap_or(input.len());

        match non_whitespace {
            0 => None,
            len => Some(Token::new(Whitespace::Other, len)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whitespace() {
        assert_eq!(
            Whitespace::parse(" "),
            Some(Token::new(Whitespace::Other, 1))
        );
        assert_eq!(
            Whitespace::parse("\t"),
            Some(Token::new(Whitespace::Other, 1))
        );
        assert_eq!(
            Whitespace::parse("\n"),
            Some(Token::new(Whitespace::NewLine, 1))
        );
        assert_eq!(
            Whitespace::parse(" \t\n"),
            Some(Token::new(Whitespace::Other, 2))
        );
        assert_eq!(Whitespace::parse("a \t\n"), None);
    }
}
