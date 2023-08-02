use super::TokenKind::{TextDouble, TextSingle, TextUnquoted};
use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key;

impl Parse for Key {
    fn parse(input: &str) -> Option<Token> {
        if input.starts_with('\'') {
            let (idx, _) = input
                .char_indices()
                .find(|(i, c)| *i != 0 && *c == '\'' && !input[..*i].ends_with('\\'))?;
            Some(Token::new(TextSingle, idx + 1))
        } else if input.starts_with('"') {
            let (idx, _) = input
                .char_indices()
                .find(|(i, c)| *i != 0 && *c == '"' && !input[..*i].ends_with('\\'))?;
            Some(Token::new(TextDouble, idx + 1))
        } else {
            let terminators = [',', ':', '[', ']', '{', '}'];
            let len = input
                .find(|c: char| c.is_whitespace() || terminators.contains(&c))
                .unwrap_or(input.len());
            Some(Token::new(TextUnquoted, len))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn single_quote() {
        assert_eq!(Key::parse("''"), Some(Token::new(TextSingle, 2)));
        assert_eq!(Key::parse("'foo'"), Some(Token::new(TextSingle, 5)));
        assert_eq!(Key::parse("'a'b"), Some(Token::new(TextSingle, 3)));
        assert_eq!(Key::parse(r#"'a\'b'"#), Some(Token::new(TextSingle, 6)));
    }

    #[test]
    fn double_quote() {
        assert_eq!(Key::parse(r#""""#), Some(Token::new(TextDouble, 2)));
        assert_eq!(Key::parse(r#""foo""#), Some(Token::new(TextDouble, 5)));
        assert_eq!(Key::parse(r#""a"b"#), Some(Token::new(TextDouble, 3)));
        assert_eq!(Key::parse(r#""a\"b""#), Some(Token::new(TextDouble, 6)));
    }

    #[test]
    fn unquoted() {
        assert_eq!(Key::parse("foo"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo.bar"), Some(Token::new(TextUnquoted, 7)));
        assert_eq!(Key::parse("foo_bar"), Some(Token::new(TextUnquoted, 7)));
        assert_eq!(Key::parse("foo "), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo\t"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(
            Key::parse(indoc! {"
                foo
                bar
            "}),
            Some(Token::new(TextUnquoted, 3))
        );
        assert_eq!(Key::parse("foo bar"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo,bar"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo:bar"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo[bar"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo]bar"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo{bar"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Key::parse("foo}bar"), Some(Token::new(TextUnquoted, 3)));
    }

    #[test]
    fn unclosed() {
        assert_eq!(Key::parse("'foo"), None);
        assert_eq!(Key::parse(r#""foo"#), None);
    }
}
