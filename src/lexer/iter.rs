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
    pub fn parse(input: &'a str) -> Self {
        Self {
            input,
            cursor: Cursor::default(),
            text_mode: TextMode::Key,
            done: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span {
    pub kind: TokenKind,
    pub start: Cursor,
    pub len: usize,
}

impl Span {
    pub fn new(kind: TokenKind, start: Cursor, len: usize) -> Self {
        Span { kind, start, len }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Span;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            if self.done {
                return None;
            } else {
                self.done = true;
                return Some(Span::new(TokenKind::Eof, self.cursor, 0));
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
        Some(Span::new(token.kind, prev_cursor, token.len))
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
        let expected = [
            Span::new(TokenKind::TextUnquoted, Cursor::new(1, 1, 0), 3),
            Span::new(TokenKind::Colon, Cursor::new(1, 4, 3), 1),
            Span::new(TokenKind::Whitespace, Cursor::new(1, 5, 4), 1),
            Span::new(TokenKind::TextUnquoted, Cursor::new(1, 6, 5), 3),
            Span::new(TokenKind::NewLine, Cursor::new(1, 9, 8), 1),
            Span::new(TokenKind::TextSingle, Cursor::new(2, 1, 9), 5),
            Span::new(TokenKind::Colon, Cursor::new(2, 6, 14), 1),
            Span::new(TokenKind::Whitespace, Cursor::new(2, 7, 15), 1),
            Span::new(TokenKind::TextUnquoted, Cursor::new(2, 8, 16), 19),
            Span::new(TokenKind::NewLine, Cursor::new(2, 27, 35), 1),
            Span::new(TokenKind::LineComment, Cursor::new(3, 1, 36), 10),
            Span::new(TokenKind::NewLine, Cursor::new(3, 11, 46), 1),
            Span::new(TokenKind::TextUnquoted, Cursor::new(4, 1, 47), 3),
            Span::new(TokenKind::Colon, Cursor::new(4, 4, 50), 1),
            Span::new(TokenKind::Whitespace, Cursor::new(4, 5, 51), 1),
            Span::new(TokenKind::TextDouble, Cursor::new(4, 6, 52), 7),
            Span::new(TokenKind::Whitespace, Cursor::new(4, 13, 59), 1),
            Span::new(TokenKind::LineComment, Cursor::new(4, 14, 60), 10),
            Span::new(TokenKind::NewLine, Cursor::new(4, 24, 70), 1),
            Span::new(TokenKind::TextUnquoted, Cursor::new(5, 1, 71), 9),
            Span::new(TokenKind::Colon, Cursor::new(5, 10, 80), 1),
            Span::new(TokenKind::Whitespace, Cursor::new(5, 11, 81), 1),
            Span::new(TokenKind::TextMulti, Cursor::new(5, 12, 82), 34),
            Span::new(TokenKind::NewLine, Cursor::new(9, 4, 116), 1),
            Span::new(TokenKind::Eof, Cursor::new(10, 1, 117), 0),
        ];

        let tokens: Vec<_> = Tokens::parse(input).collect();
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
            Span::new(TokenKind::TextUnquoted, Cursor::new(1, 1, 0), 3),
            Span::new(TokenKind::Colon, Cursor::new(1, 4, 3), 1),
            Span::new(TokenKind::Whitespace, Cursor::new(1, 5, 4), 1),
            Span::new(TokenKind::TextUnquoted, Cursor::new(1, 6, 5), 9),
            Span::new(TokenKind::Eof, Cursor::new(1, 15, 14), 0),
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
            Span::new(TokenKind::TextUnquoted, Cursor::new(1, 1, 0), 2),
            Span::new(TokenKind::Colon, Cursor::new(1, 3, 2), 1),
            Span::new(TokenKind::Whitespace, Cursor::new(1, 4, 3), 1),
            Span::new(TokenKind::TextSingle, Cursor::new(1, 5, 4), 5),
            Span::new(TokenKind::Eof, Cursor::new(1, 10, 9), 0),
        ];

        for (got, expected) in iter::zip(tokens, expected) {
            assert_eq!(got, expected);
        }
    }
}
