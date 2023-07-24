use super::boolean::Boolean;
use super::comment::Comment;
use super::key::Key;
use super::null::Null;
use super::number::Number;
use super::symbol::Symbol;
use super::text::Text;
use super::whitespace::Whitespace;
use super::{Parse, Token, TokenKind};

pub struct Tokens<'a> {
    input: &'a str,
    cursor: Cursor,
    text_mode: TextMode,
    done: bool,
}

impl<'a> Tokens<'a> {
    /// Zero-length EOF token returned at the end of the file.
    const EOF: Token = Token {
        kind: TokenKind::Eof,
        len: 0,
    };

    pub fn parse(input: &'a str) -> Self {
        Self {
            input,
            cursor: Cursor::default(),
            text_mode: TextMode::Key,
            done: false,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = (Cursor, Token);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            if self.done {
                return None;
            } else {
                self.done = true;
                return Some((self.cursor, Self::EOF));
            }
        }

        let token = next_token(self.input, self.text_mode)?;

        self.text_mode = match token.kind {
            TokenKind::Colon => TextMode::Value,
            TokenKind::Whitespace
            | TokenKind::NewLine
            | TokenKind::LineComment
            | TokenKind::HashComment
            | TokenKind::BlockComment => self.text_mode,
            _ => TextMode::Key,
        };

        let prev_cursor = self.cursor;

        // Update the cursor to the next token.
        self.cursor.byte_offset += token.len;
        self.cursor.line += self.input[..token.len].matches('\n').count();
        match self.input[..token.len].rfind('\n') {
            Some(x) => self.cursor.column = token.len - x,
            None => self.cursor.column += token.len,
        }

        // Update the input to point to the next token.
        self.input = &self.input[token.len..];

        // Ensure we give the cursor for _this_ token and not the next.
        Some((prev_cursor, token))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TextMode {
    Key,
    Value,
}

fn next_token(input: &str, text_mode: TextMode) -> Option<Token> {
    // The parser behaves differently depending on whether it's in `Key` or
    // `Value` mode. Strings take priority over Booleans, numbers, and `null`
    // for keys, whereas strings are parsed _last_ for values.
    let parsers = match text_mode {
        TextMode::Key => [
            Comment::parse,
            Symbol::parse,
            Whitespace::parse,
            Key::parse,
            Boolean::parse,
            Null::parse,
            Number::parse,
        ],
        TextMode::Value => [
            Comment::parse,
            Symbol::parse,
            Whitespace::parse,
            Boolean::parse,
            Null::parse,
            Number::parse,
            Text::parse,
        ],
    };

    parsers.into_iter().find_map(|p| p(input))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            line: 1,
            column: 1,
            byte_offset: 0,
        }
    }
}

impl Cursor {
    pub fn new(line: usize, column: usize, byte_offset: usize) -> Self {
        Cursor {
            line,
            column,
            byte_offset,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use std::iter;

    /// Test a typical file which uses a few different Hjson features.
    #[test]
    fn typical() {
        let input = indoc! {r#"
            foo: bar
            'baz': https://example.com
            // comment
            key: "value" // comment
            multiline: '''
                lots
                of '
                text
            '''
        "#};
        let expected_tokens = [
            Token::new(TokenKind::TextUnquoted, 3),
            Token::new(TokenKind::Colon, 1),
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::TextUnquoted, 3),
            Token::new(TokenKind::NewLine, 1),
            Token::new(TokenKind::TextSingle, 5),
            Token::new(TokenKind::Colon, 1),
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::TextUnquoted, 19),
            Token::new(TokenKind::NewLine, 1),
            Token::new(TokenKind::LineComment, 10),
            Token::new(TokenKind::NewLine, 1),
            Token::new(TokenKind::TextUnquoted, 3),
            Token::new(TokenKind::Colon, 1),
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::TextDouble, 7),
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::LineComment, 10),
            Token::new(TokenKind::NewLine, 1),
            Token::new(TokenKind::TextUnquoted, 9),
            Token::new(TokenKind::Colon, 1),
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::TextMulti, 34),
            Token::new(TokenKind::NewLine, 1),
            Token::new(TokenKind::Eof, 0),
        ];
        let expected_cursors = [
            Cursor::new(1, 1, 0),
            Cursor::new(1, 4, 3),
            Cursor::new(1, 5, 4),
            Cursor::new(1, 6, 5),
            Cursor::new(1, 9, 8),
            Cursor::new(2, 1, 9),
            Cursor::new(2, 6, 14),
            Cursor::new(2, 7, 15),
            Cursor::new(2, 8, 16),
            Cursor::new(2, 27, 35),
            Cursor::new(3, 1, 36),
            Cursor::new(3, 11, 46),
            Cursor::new(4, 1, 47),
            Cursor::new(4, 4, 50),
            Cursor::new(4, 5, 51),
            Cursor::new(4, 6, 52),
            Cursor::new(4, 13, 59),
            Cursor::new(4, 14, 60),
            Cursor::new(4, 24, 70),
            Cursor::new(5, 1, 71),
            Cursor::new(5, 10, 80),
            Cursor::new(5, 11, 81),
            Cursor::new(5, 12, 82),
            Cursor::new(9, 4, 116),
            Cursor::new(10, 1, 117),
        ];

        let tokens: Vec<_> = Tokens::parse(input).collect();
        let expected = iter::zip(expected_cursors, expected_tokens);
        for (got, expected) in iter::zip(tokens, expected) {
            assert_eq!(got, expected);
        }
    }

    /// Test that checks unquoted string values that look like numbers.
    #[test]
    fn string_not_number() {
        let input = "foo: 20 apples";

        let tokens: Vec<_> = Tokens::parse(input).collect();
        let expected = [
            (Cursor::new(1, 1, 0), Token::new(TokenKind::TextUnquoted, 3)),
            (Cursor::new(1, 4, 3), Token::new(TokenKind::Colon, 1)),
            (Cursor::new(1, 5, 4), Token::new(TokenKind::Whitespace, 1)),
            (Cursor::new(1, 6, 5), Token::new(TokenKind::TextUnquoted, 9)),
            (Cursor::new(1, 15, 14), Token::new(TokenKind::Eof, 0)),
        ];

        for (got, expected) in iter::zip(tokens, expected) {
            assert_eq!(got, expected);
        }
    }

    /// Test that checks numbers used as keys are actually parsed as unquoted strings.
    #[test]
    fn number_key() {
        let input = "10: 'foo'";

        let tokens: Vec<_> = Tokens::parse(input).collect();
        let expected = [
            (Cursor::new(1, 1, 0), Token::new(TokenKind::TextUnquoted, 2)),
            (Cursor::new(1, 3, 2), Token::new(TokenKind::Colon, 1)),
            (Cursor::new(1, 4, 3), Token::new(TokenKind::Whitespace, 1)),
            (Cursor::new(1, 5, 4), Token::new(TokenKind::TextSingle, 5)),
            (Cursor::new(1, 10, 9), Token::new(TokenKind::Eof, 0)),
        ];

        for (got, expected) in iter::zip(tokens, expected) {
            assert_eq!(got, expected);
        }
    }
}
