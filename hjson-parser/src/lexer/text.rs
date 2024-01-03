use crate::token::{Token, TokenKind};

/// Parse valid Hjson text in the "value" context (as opposed to "key") context.
///
/// Text can be either:
///
/// 1. Multi-line: `'''\n'''`.
/// 2. Single-quoted: `'foo'`.
/// 3. Double-quoted: `"foo"`.
/// 4. Unquoted: `foo bar!`.
///
/// Note that the unqouted text lexer matches _anything_ up to a new-line (or
/// end of input). You should run this lexer after running other possible
/// lexers, as they aren't mutually exclusive.
pub fn parse(input: &str) -> Option<Token> {
    if let Some(input) = input.strip_prefix("'''") {
        let len = input.find("'''")? + 6;
        Some(TokenKind::TextMulti.with_len(len))
    } else if input.starts_with('\'') {
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
        let eol = input.find('\n').unwrap_or(input.len());
        let len = input[..eol].trim_end().len();
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
    fn multi_line() {
        assert_eq!(parse("'''foo'''"), Some(TokenKind::TextMulti.with_len(9)));
        assert_eq!(
            parse(indoc! {"
                '''
                foo
                '''
            "}),
            Some(TokenKind::TextMulti.with_len(11))
        );
        assert_eq!(parse("'''a'''b"), Some(TokenKind::TextMulti.with_len(7)));
        assert_eq!(
            parse(r#"'''a\'b'''"#),
            Some(TokenKind::TextMulti.with_len(10))
        );
    }

    #[test]
    fn unquoted() {
        assert_eq!(parse("foo"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo "), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(parse("foo\t"), Some(TokenKind::TextUnquoted.with_len(3)));
        assert_eq!(
            parse(indoc! {"
                foo
                bar
            "}),
            Some(TokenKind::TextUnquoted.with_len(3))
        );
    }

    #[test]
    fn unclosed() {
        assert_eq!(parse("'foo"), None);
        assert_eq!(parse(r#""foo"#), None);
        assert_eq!(parse("'''foo"), None);
    }
}
