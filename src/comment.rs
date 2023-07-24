use super::TokenKind::{BlockComment, HashComment, LineComment};
use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Comment;

impl Parse for Comment {
    fn parse(input: &str) -> Option<Token> {
        if input.starts_with("//") {
            let len = input.find('\n').unwrap_or(input.len());
            Some(Token::new(LineComment, len))
        } else if let Some(input) = input.strip_prefix("/*") {
            let len = input.find("*/")? + 4;
            Some(Token::new(BlockComment, len))
        } else if input.starts_with('#') {
            let len = input.find('\n').unwrap_or(input.len());
            Some(Token::new(HashComment, len))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn line() {
        assert_eq!(Comment::parse("//"), Some(Token::new(LineComment, 2)));
        assert_eq!(Comment::parse("// foo"), Some(Token::new(LineComment, 6)));
        assert_eq!(
            Comment::parse(indoc! {"
                //
                bar
            "}),
            Some(Token::new(LineComment, 2))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                // foo
                bar
            "}),
            Some(Token::new(LineComment, 6))
        );
    }

    #[test]
    fn block() {
        assert_eq!(Comment::parse("/**/"), Some(Token::new(BlockComment, 4)));
        assert_eq!(
            Comment::parse("/* foo */"),
            Some(Token::new(BlockComment, 9))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                /* foo
                bar */
            "}),
            Some(Token::new(BlockComment, 13))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                /* foo */
                bar
            "}),
            Some(Token::new(BlockComment, 9))
        );
    }

    #[test]
    fn hash() {
        assert_eq!(Comment::parse("#"), Some(Token::new(HashComment, 1)));
        assert_eq!(Comment::parse("# foo"), Some(Token::new(HashComment, 5)));
        assert_eq!(
            Comment::parse(indoc! {"
                #
                bar
            "}),
            Some(Token::new(HashComment, 1))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                # foo
                bar
            "}),
            Some(Token::new(HashComment, 5))
        );
    }

    #[test]
    fn unclosed() {
        assert_eq!(Comment::parse("/* foo"), None);
        assert_eq!(Comment::parse("/*/"), None);
    }
}
