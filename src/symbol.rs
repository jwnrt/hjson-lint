use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
}

impl Parse for Symbol {
    fn parse(input: &str) -> Option<Token> {
        let symbol = match input.chars().next()? {
            '{' => Symbol::OpenBrace,
            '}' => Symbol::CloseBrace,
            '[' => Symbol::OpenBracket,
            ']' => Symbol::OpenBracket,
            ':' => Symbol::Colon,
            ',' => Symbol::Comma,
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
            ("{", Symbol::OpenBrace),
            ("}", Symbol::CloseBrace),
            ("[", Symbol::OpenBracket),
            ("]", Symbol::OpenBracket),
            (":", Symbol::Colon),
            (",", Symbol::Comma),
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
