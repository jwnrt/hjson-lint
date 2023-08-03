use std::iter;

use crate::boolean::Boolean;
use crate::comment::Comment;
use crate::key::Key;
use crate::null::Null;
use crate::number::Number;
use crate::symbol::Symbol;
use crate::text::Text;
use crate::whitespace::Whitespace;
use crate::{Parse, Token, TokenKind};

pub struct Tokens<'a> {
    input: &'a str,
    cursor: Cursor,
    text_mode: TextMode,
}

impl<'a> Tokens<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            input,
            cursor: Cursor::default(),
            text_mode: TextMode::Key,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = (Cursor, Token);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let token = next_token(self.input, self.text_mode)?;

        self.text_mode = match token.kind {
            TokenKind::Colon => TextMode::Value,
            TokenKind::Whitespace => self.text_mode,
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
    let parsers = [
        Boolean::parse,
        Comment::parse,
        Null::parse,
        Number::parse,
        Symbol::parse,
        Whitespace::parse,
    ];

    let text_parser = match text_mode {
        TextMode::Key => Key::parse,
        TextMode::Value => Text::parse,
    };
    let mut parser_chain = parsers.into_iter().chain(iter::once(text_parser));

    parser_chain.find_map(|p| p(input))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    line: usize,
    column: usize,
    byte_offset: usize,
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
        ];

        let tokens: Vec<_> = Tokens::parse(input).collect();
        let token_spans = iter::zip(expected_cursors, expected_tokens);
        for (expected, got) in iter::zip(token_spans, tokens) {
            assert_eq!(expected, got);
        }
    }
}
