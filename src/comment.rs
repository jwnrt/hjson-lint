use super::{Parse, Token};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comment {
    Line,
    Block,
    Hash,
}

impl Parse for Comment {
    fn parse(input: &str) -> Option<Token> {
        if input.starts_with("//") {
            let len = input.find('\n').unwrap_or(input.len());
            Some(Token::new(Comment::Line, len))
        } else if let Some(input) = input.strip_prefix("/*") {
            let len = input.find("*/")? + 4;
            Some(Token::new(Comment::Block, len))
        } else if input.starts_with('#') {
            let len = input.find('\n').unwrap_or(input.len());
            Some(Token::new(Comment::Hash, len))
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
        assert_eq!(Comment::parse("//"), Some(Token::new(Comment::Line, 2)));
        assert_eq!(Comment::parse("// foo"), Some(Token::new(Comment::Line, 6)));
        assert_eq!(
            Comment::parse(indoc! {"
                //
                bar
            "}),
            Some(Token::new(Comment::Line, 2))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                // foo
                bar
            "}),
            Some(Token::new(Comment::Line, 6))
        );
    }

    #[test]
    fn block() {
        assert_eq!(Comment::parse("/**/"), Some(Token::new(Comment::Block, 4)));
        assert_eq!(
            Comment::parse("/* foo */"),
            Some(Token::new(Comment::Block, 9))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                /* foo
                bar */
            "}),
            Some(Token::new(Comment::Block, 13))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                /* foo */
                bar
            "}),
            Some(Token::new(Comment::Block, 9))
        );
    }

    #[test]
    fn hash() {
        assert_eq!(Comment::parse("#"), Some(Token::new(Comment::Hash, 1)));
        assert_eq!(Comment::parse("# foo"), Some(Token::new(Comment::Hash, 5)));
        assert_eq!(
            Comment::parse(indoc! {"
                #
                bar
            "}),
            Some(Token::new(Comment::Hash, 1))
        );
        assert_eq!(
            Comment::parse(indoc! {"
                # foo
                bar
            "}),
            Some(Token::new(Comment::Hash, 5))
        );
    }

    #[test]
    fn unclosed() {
        assert_eq!(Comment::parse("/* foo"), None);
        assert_eq!(Comment::parse("/*/"), None);
    }
}
