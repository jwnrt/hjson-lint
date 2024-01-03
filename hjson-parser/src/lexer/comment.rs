use crate::token::Token;
use crate::token::TokenKind::{BlockComment, HashComment, LineComment};

/// Parse line (`//`), block (`/* */`), and hash (`#`) comments.
pub fn parse(input: &str) -> Option<Token> {
    if input.starts_with("//") {
        let len = input.find('\n').unwrap_or(input.len());
        Some(LineComment.with_len(len))
    } else if let Some(input) = input.strip_prefix("/*") {
        let len = input.find("*/")? + 4;
        Some(BlockComment.with_len(len))
    } else if input.starts_with('#') {
        let len = input.find('\n').unwrap_or(input.len());
        Some(HashComment.with_len(len))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn line() {
        assert_eq!(parse("//"), Some(LineComment.with_len(2)));
        assert_eq!(parse("// foo"), Some(LineComment.with_len(6)));
        assert_eq!(
            parse(indoc! {"
                //
                bar
            "}),
            Some(LineComment.with_len(2))
        );
        assert_eq!(
            parse(indoc! {"
                // foo
                bar
            "}),
            Some(LineComment.with_len(6))
        );
    }

    #[test]
    fn block() {
        assert_eq!(parse("/**/"), Some(BlockComment.with_len(4)));
        assert_eq!(parse("/* foo */"), Some(BlockComment.with_len(9)));
        assert_eq!(
            parse(indoc! {"
                /* foo
                bar */
            "}),
            Some(BlockComment.with_len(13))
        );
        assert_eq!(
            parse(indoc! {"
                /* foo */
                bar
            "}),
            Some(BlockComment.with_len(9))
        );
    }

    #[test]
    fn hash() {
        assert_eq!(parse("#"), Some(HashComment.with_len(1)));
        assert_eq!(parse("# foo"), Some(HashComment.with_len(5)));
        assert_eq!(
            parse(indoc! {"
                #
                bar
            "}),
            Some(HashComment.with_len(1))
        );
        assert_eq!(
            parse(indoc! {"
                # foo
                bar
            "}),
            Some(HashComment.with_len(5))
        );
    }

    #[test]
    fn unclosed() {
        assert_eq!(parse("/* foo"), None);
        assert_eq!(parse("/*/"), None);
    }
}
