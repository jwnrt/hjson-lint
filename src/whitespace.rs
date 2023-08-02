use super::{Parse, Token, TokenKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Whitespace;

impl Parse for Whitespace {
    fn parse(input: &str) -> Option<Token> {
        if input.starts_with('\n') {
            return Some(Token::new(TokenKind::NewLine, 1));
        }

        let non_whitespace = input
            .find(|c: char| c == '\n' || !c.is_whitespace())
            .unwrap_or(input.len());

        match non_whitespace {
            0 => None,
            len => Some(Token::new(TokenKind::Whitespace, len)),
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
            Some(Token::new(TokenKind::Whitespace, 1))
        );
        assert_eq!(
            Whitespace::parse("\t"),
            Some(Token::new(TokenKind::Whitespace, 1))
        );
        assert_eq!(
            Whitespace::parse("\n"),
            Some(Token::new(TokenKind::NewLine, 1))
        );
        assert_eq!(
            Whitespace::parse(" \t\n"),
            Some(Token::new(TokenKind::Whitespace, 2))
        );
        assert_eq!(Whitespace::parse("a \t\n"), None);
    }
}
