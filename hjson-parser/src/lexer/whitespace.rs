use crate::token::{Token, TokenKind};

/// Parse any whitespace (new-lines, tabs, spaces, end of file).
///
/// Note that tabs, spaces etc are merged into one token, while new-lines are
/// each given as separate tokens. This is to aid with line number tracking,
/// etc.
///
/// Note also that the EOF token has zero length.
pub fn parse(input: &str) -> Option<Token> {
    if input.is_empty() {
        return Some(TokenKind::Eof.with_len(0));
    }

    if input.starts_with('\n') {
        return Some(TokenKind::NewLine.with_len(1));
    }

    let non_whitespace = input
        .find(|c: char| c == '\n' || !c.is_whitespace())
        .unwrap_or(input.len());

    match non_whitespace {
        0 => None,
        len => Some(TokenKind::Whitespace.with_len(len)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn whitespace() {
        assert_eq!(parse(""), Some(TokenKind::Eof.with_len(0)));
        assert_eq!(parse(" "), Some(TokenKind::Whitespace.with_len(1)));
        assert_eq!(parse("\t"), Some(TokenKind::Whitespace.with_len(1)));
        assert_eq!(parse("\n"), Some(TokenKind::NewLine.with_len(1)));
        assert_eq!(parse(" \t\n"), Some(TokenKind::Whitespace.with_len(2)));
        assert_eq!(parse("a \t\n"), None);
    }
}
