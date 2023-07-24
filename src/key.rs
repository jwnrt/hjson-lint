use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Single,
    Double,
    Unquoted,
}

impl Parse for Key {
    fn parse(input: &str) -> Option<Token> {
        if input.starts_with('\'') {
            let (idx, _) = input
                .char_indices()
                .find(|(i, c)| *i != 0 && *c == '\'' && !input[..*i].ends_with('\\'))?;
            Some(Token::new(Key::Single, idx + 1))
        } else if input.starts_with('"') {
            let (idx, _) = input
                .char_indices()
                .find(|(i, c)| *i != 0 && *c == '"' && !input[..*i].ends_with('\\'))?;
            Some(Token::new(Key::Double, idx + 1))
        } else {
            let terminators = [',', ':', '[', ']', '{', '}'];
            let len = input
                .find(|c: char| c.is_whitespace() || terminators.contains(&c))
                .unwrap_or(input.len());
            Some(Token::new(Key::Unquoted, len))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn single_quote() {
        assert_eq!(Key::parse("''"), Some(Token::new(Key::Single, 2)));
        assert_eq!(Key::parse("'foo'"), Some(Token::new(Key::Single, 5)));
        assert_eq!(Key::parse("'a'b"), Some(Token::new(Key::Single, 3)));
        assert_eq!(Key::parse(r#"'a\'b'"#), Some(Token::new(Key::Single, 6)));
    }

    #[test]
    fn double_quote() {
        assert_eq!(Key::parse(r#""""#), Some(Token::new(Key::Double, 2)));
        assert_eq!(Key::parse(r#""foo""#), Some(Token::new(Key::Double, 5)));
        assert_eq!(Key::parse(r#""a"b"#), Some(Token::new(Key::Double, 3)));
        assert_eq!(Key::parse(r#""a\"b""#), Some(Token::new(Key::Double, 6)));
    }

    #[test]
    fn unquoted() {
        assert_eq!(Key::parse("foo"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo.bar"), Some(Token::new(Key::Unquoted, 7)));
        assert_eq!(Key::parse("foo_bar"), Some(Token::new(Key::Unquoted, 7)));
        assert_eq!(Key::parse("foo "), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo\t"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(
            Key::parse(indoc! {"
                foo
                bar
            "}),
            Some(Token::new(Key::Unquoted, 3))
        );
        assert_eq!(Key::parse("foo bar"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo,bar"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo:bar"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo[bar"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo]bar"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo{bar"), Some(Token::new(Key::Unquoted, 3)));
        assert_eq!(Key::parse("foo}bar"), Some(Token::new(Key::Unquoted, 3)));
    }

    #[test]
    fn unclosed() {
        assert_eq!(Key::parse("'foo"), None);
        assert_eq!(Key::parse(r#""foo"#), None);
    }
}
