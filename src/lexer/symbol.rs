use super::{Parse, Token, TokenKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol;

impl Parse for Symbol {
    fn parse(input: &str) -> Option<Token> {
        let symbol = match input.chars().next()? {
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            _ => return None,
        };

        Some(Token::new(symbol, 1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid() {
        let symbols = [
            ("{", TokenKind::OpenBrace),
            ("}", TokenKind::CloseBrace),
            ("[", TokenKind::OpenBracket),
            ("]", TokenKind::CloseBracket),
            (":", TokenKind::Colon),
            (",", TokenKind::Comma),
        ];

        for (s, symbol) in symbols {
            assert_eq!(Symbol::parse(s), Some(Token::new(symbol, 1)));
        }
    }

    #[test]
    fn invalid() {
        let invalid = ["!", " {", "x"];
        for invalid in invalid {
            assert_eq!(Symbol::parse(invalid), None);
        }
    }
}
