use crate::token::{Token, TokenKind};

/// Parse Hjson keywords: `true`, `false`, and `null`.
pub fn parse(input: &str) -> Option<Token> {
    if input.starts_with("true") {
        Some(TokenKind::Bool.with_len(4))
    } else if input.starts_with("false") {
        Some(TokenKind::Bool.with_len(5))
    } else if input.starts_with("null") {
        Some(TokenKind::Null.with_len(4))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn boolean() {
        assert_eq!(parse("true"), Some(TokenKind::Bool.with_len(4)));
        assert_eq!(parse("false"), Some(TokenKind::Bool.with_len(5)));
        assert_eq!(parse("true "), Some(TokenKind::Bool.with_len(4)));
    }

    #[test]
    fn null() {
        assert_eq!(parse("null"), Some(TokenKind::Null.with_len(4)));
        assert_eq!(parse("null "), Some(TokenKind::Null.with_len(4)));
    }

    #[test]
    fn invalid() {
        assert_eq!(parse("foo"), None);
        assert_eq!(parse(" true"), None);
        assert_eq!(parse(""), None);
    }
}
