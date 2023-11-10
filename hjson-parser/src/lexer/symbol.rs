use crate::token::{Token, TokenKind};

/// Parse valid Hjson symbols: `{}[]:,`.
pub fn parse(input: &str) -> Option<Token> {
    let symbol = match input.chars().next()? {
        '{' => TokenKind::LBrace,
        '}' => TokenKind::RBrace,
        '[' => TokenKind::LBracket,
        ']' => TokenKind::RBracket,
        ':' => TokenKind::Colon,
        ',' => TokenKind::Comma,
        _ => return None,
    };

    Some(symbol.with_len(1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid() {
        let symbols = [
            ("{", TokenKind::LBrace),
            ("}", TokenKind::RBrace),
            ("[", TokenKind::LBracket),
            ("]", TokenKind::RBracket),
            (":", TokenKind::Colon),
            (",", TokenKind::Comma),
        ];

        for (s, symbol) in symbols {
            assert_eq!(parse(s), Some(symbol.with_len(1)));
        }
    }

    #[test]
    fn invalid() {
        let invalid = ["!", " {", "x"];
        for invalid in invalid {
            assert_eq!(parse(invalid), None);
        }
    }
}
