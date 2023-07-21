use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Text {
    Single,
    Double,
    Multi,
    Unquoted,
}

impl Parse for Text {
    fn parse(input: &str) -> Option<Token> {
        if let Some(input) = input.strip_prefix("'''") {
            let len = input.find("'''")? + 6;
            Some(Token::new(Text::Multi, len))
        } else if input.starts_with('\'') {
            let (idx, _) = input
                .char_indices()
                .find(|(i, c)| *i != 0 && *c == '\'' && !input[..*i].ends_with('\\'))?;
            Some(Token::new(Text::Single, idx + 1))
        } else if input.starts_with('"') {
            let (idx, _) = input
                .char_indices()
                .find(|(i, c)| *i != 0 && *c == '"' && !input[..*i].ends_with('\\'))?;
            Some(Token::new(Text::Double, idx + 1))
        } else {
            let eol = input.find('\n').unwrap_or(input.len());
            let len = input[..eol].trim_end().len();
            Some(Token::new(Text::Unquoted, len))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn single_quote() {
        assert_eq!(Text::parse("''"), Some(Token::new(Text::Single, 2)));
        assert_eq!(Text::parse("'foo'"), Some(Token::new(Text::Single, 5)));
        assert_eq!(Text::parse("'a'b"), Some(Token::new(Text::Single, 3)));
        assert_eq!(Text::parse(r#"'a\'b'"#), Some(Token::new(Text::Single, 6)));
    }

    #[test]
    fn double_quote() {
        assert_eq!(Text::parse(r#""""#), Some(Token::new(Text::Double, 2)));
        assert_eq!(Text::parse(r#""foo""#), Some(Token::new(Text::Double, 5)));
        assert_eq!(Text::parse(r#""a"b"#), Some(Token::new(Text::Double, 3)));
        assert_eq!(Text::parse(r#""a\"b""#), Some(Token::new(Text::Double, 6)));
    }

    #[test]
    fn multi_line() {
        assert_eq!(Text::parse("'''foo'''"), Some(Token::new(Text::Multi, 9)));
        assert_eq!(
            Text::parse(indoc! {"
                '''
                foo
                '''
            "}),
            Some(Token::new(Text::Multi, 11))
        );
        assert_eq!(Text::parse("'''a'''b"), Some(Token::new(Text::Multi, 7)));
        assert_eq!(
            Text::parse(r#"'''a\'b'''"#),
            Some(Token::new(Text::Multi, 10))
        );
    }

    #[test]
    fn unquoted() {
        assert_eq!(Text::parse("foo"), Some(Token::new(Text::Unquoted, 3)));
        assert_eq!(Text::parse("foo "), Some(Token::new(Text::Unquoted, 3)));
        assert_eq!(Text::parse("foo\t"), Some(Token::new(Text::Unquoted, 3)));
        assert_eq!(
            Text::parse(indoc! {"
                foo
                bar
            "}),
            Some(Token::new(Text::Unquoted, 3))
        );
    }

    #[test]
    fn unclosed() {
        assert_eq!(Text::parse("'foo"), None);
        assert_eq!(Text::parse(r#""foo"#), None);
        assert_eq!(Text::parse("'''foo"), None);
    }
}
