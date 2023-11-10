use super::TokenKind::{TextDouble, TextMulti, TextSingle, TextUnquoted};
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
            Some(Token::new(TextMulti, len))
        } else if input.starts_with('\'') {
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
            let eol = input.find('\n').unwrap_or(input.len());
            let len = input[..eol].trim_end().len();
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
        assert_eq!(Text::parse("''"), Some(Token::new(TextSingle, 2)));
        assert_eq!(Text::parse("'foo'"), Some(Token::new(TextSingle, 5)));
        assert_eq!(Text::parse("'a'b"), Some(Token::new(TextSingle, 3)));
        assert_eq!(Text::parse(r#"'a\'b'"#), Some(Token::new(TextSingle, 6)));
    }

    #[test]
    fn double_quote() {
        assert_eq!(Text::parse(r#""""#), Some(Token::new(TextDouble, 2)));
        assert_eq!(Text::parse(r#""foo""#), Some(Token::new(TextDouble, 5)));
        assert_eq!(Text::parse(r#""a"b"#), Some(Token::new(TextDouble, 3)));
        assert_eq!(Text::parse(r#""a\"b""#), Some(Token::new(TextDouble, 6)));
    }

    #[test]
    fn multi_line() {
        assert_eq!(Text::parse("'''foo'''"), Some(Token::new(TextMulti, 9)));
        assert_eq!(
            Text::parse(indoc! {"
                '''
                foo
                '''
            "}),
            Some(Token::new(TextMulti, 11))
        );
        assert_eq!(Text::parse("'''a'''b"), Some(Token::new(TextMulti, 7)));
        assert_eq!(
            Text::parse(r#"'''a\'b'''"#),
            Some(Token::new(TextMulti, 10))
        );
    }

    #[test]
    fn unquoted() {
        assert_eq!(Text::parse("foo"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Text::parse("foo "), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(Text::parse("foo\t"), Some(Token::new(TextUnquoted, 3)));
        assert_eq!(
            Text::parse(indoc! {"
                foo
                bar
            "}),
            Some(Token::new(TextUnquoted, 3))
        );
    }

    #[test]
    fn unclosed() {
        assert_eq!(Text::parse("'foo"), None);
        assert_eq!(Text::parse(r#""foo"#), None);
        assert_eq!(Text::parse("'''foo"), None);
    }
}
