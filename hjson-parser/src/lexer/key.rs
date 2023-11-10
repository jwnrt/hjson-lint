use crate::token::{Token, TokenKind};

/// Parse text in the "key" context, meaning as if it appeared as a key in a
/// map on the left of a `:` symbol.
///
/// This parser is different to the [`crate::lexer::text::parse`] parser in
/// that unquoted strings will be terminated at certain characters (e.g. `:`).
pub fn parse(input: &str) -> Option<Token> {
    if input.starts_with('\'') {
        let (idx, _) = input
            .char_indices()
            .find(|(i, c)| *i != 0 && *c == '\'' && !input[..*i].ends_with('\\'))?;
        Some(TokenKind::TextSingle.with_len(idx + 1))
    } else if input.starts_with('"') {
        let (idx, _) = input
            .char_indices()
            .find(|(i, c)| *i != 0 && *c == '"' && !input[..*i].ends_with('\\'))?;
        Some(TokenKind::TextDouble.with_len(idx + 1))
    } else {
        let terminators = [',', ':', '[', ']', '{', '}'];
        let len = input
            .find(|c: char| c.is_whitespace() || terminators.contains(&c))
            .unwrap_or(input.len());
        Some(TokenKind::TextUnquoted.with_len(len))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn single_quote() {
        assert_eq!(parse("''"), Some(TokenKind::TextSingle.with_len(2)));
        assert_eq!(parse("'foo'"), Some(TokenKind::TextSingle.with_len(5)));
        assert_eq!(parse("'a'b"), Some(TokenKind::TextSingle.with_len(3)));
        assert_eq!(parse(r#"'a\'b'"#), Some(TokenKind::TextSingle.with_len(6)));
    }

    #[test]
    fn double_quote() {
        assert_eq!(parse(r#""""#), Some(TokenKind::TextDouble.with_len(2)));
        assert_eq!(parse(r#""foo""#), Some(TokenKind::TextDouble.with_len(5)));
        assert_eq!(parse(r#""a"b"#), Some(TokenKind::TextDouble.with_len(3)));
        assert_eq!(parse(r#""a\"b""#), Some(TokenKind::TextDouble.with_len(6)));
    }

    #[test]
    fn unquoted() {
        assert_eq!(parse("foo"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo.bar"), Some(TokenKind::TextUnquoted.with_len(7)));
        assert_eq!(parse("foo_bar"), Some(TokenKind::TextUnquoted.with_len(7)));
        assert_eq!(parse("foo "), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo\t"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(
            parse(indoc! {"
                foo
                bar
            "}),
            Some(TokenKind::TextUnquoted.with_len(3))
        );
        assert_eq!(parse("foo bar"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo,bar"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo:bar"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo[bar"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo]bar"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo{bar"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo}bar"), Some(TokenKind::TextUnquoted.with_len(3)));
    }

    #[test]
    fn unclosed() {
        assert_eq!(parse("'foo"), None);
        assert_eq!(parse(r#""foo"#), None);
    }
}
