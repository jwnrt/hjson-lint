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
    text_mode: TextMode,
}

impl<'a> Tokens<'a> {
    pub fn parse(input: &'a str) -> Self {
        Self {
            input,
            text_mode: TextMode::Key,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let token = next_token(self.input, self.text_mode)?;

        self.text_mode = match token.kind {
            TokenKind::Symbol(Symbol::Colon) => TextMode::Value,
            TokenKind::Whitespace(_) => self.text_mode,
            _ => TextMode::Key,
        };

        self.input = &self.input[token.len..];

        Some(token)
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
        let expected = [
            Token::new(Key::Unquoted, 3),
            Token::new(Symbol::Colon, 1),
            Token::new(Whitespace::Other, 1),
            Token::new(Text::Unquoted, 3),
            Token::new(Whitespace::NewLine, 1),
            Token::new(Key::Single, 5),
            Token::new(Symbol::Colon, 1),
            Token::new(Whitespace::Other, 1),
            Token::new(Text::Unquoted, 19),
            Token::new(Whitespace::NewLine, 1),
            Token::new(Comment::Line, 10),
            Token::new(Whitespace::NewLine, 1),
            Token::new(Key::Unquoted, 3),
            Token::new(Symbol::Colon, 1),
            Token::new(Whitespace::Other, 1),
            Token::new(Text::Double, 7),
            Token::new(Whitespace::Other, 1),
            Token::new(Comment::Line, 10),
            Token::new(Whitespace::NewLine, 1),
            Token::new(Key::Unquoted, 9),
            Token::new(Symbol::Colon, 1),
            Token::new(Whitespace::Other, 1),
            Token::new(Text::Multi, 34),
            Token::new(Whitespace::NewLine, 1),
        ];

        let tokens: Vec<_> = Tokens::parse(input).collect();
        for (expected, got) in iter::zip(expected, tokens) {
            assert_eq!(expected, got);
        }
    }
}
