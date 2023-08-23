use std::error::Error;
use std::fmt::{self, Display};
use std::iter;
use std::iter::Peekable;
use std::mem;

use crate::lexer::{Cursor, Token, TokenKind, Tokens};

type ParseResult<T> = Result<T, ParseError>;

mod ast;

use ast::Node;

pub struct Parser<'a> {
    tokens: Peekable<Tokens<'a>>,
}

impl<'a> Parser<'a> {
    const HIDDEN: &[TokenKind] = &[
        TokenKind::Whitespace,
        TokenKind::NewLine,
        TokenKind::LineComment,
        TokenKind::HashComment,
        TokenKind::BlockComment,
    ];

    const HIDDEN_LINE: &[TokenKind] = &[
        TokenKind::Whitespace,
        TokenKind::LineComment,
        TokenKind::HashComment,
        TokenKind::BlockComment,
    ];

    const KEY: &[TokenKind] = &[
        TokenKind::TextSingle,
        TokenKind::TextDouble,
        TokenKind::TextUnquoted,
    ];

    const VALUE: &[TokenKind] = &[
        TokenKind::Boolean,
        TokenKind::Integer,
        TokenKind::Float,
        TokenKind::TextSingle,
        TokenKind::TextDouble,
        TokenKind::TextMulti,
        TokenKind::TextUnquoted,
        TokenKind::Null,
    ];

    pub fn parse(input: &'a str) -> ParseResult<Node<ast::Map>> {
        let tokens = Tokens::parse(input).peekable();
        let mut parser = Self { tokens };

        parser.parse_root()
    }

    fn parse_root(&mut self) -> ParseResult<Node<ast::Map>> {
        let before = self.skip(Self::HIDDEN);

        let open_brace = self
            .eat(&[TokenKind::OpenBrace])
            .map(|token| Node::new(Vec::new(), token, self.skip(Self::HIDDEN_LINE)));

        let members = self.parse_map_members()?;

        let (close_brace, after) = if open_brace.is_some() {
            // Explicit close brace.
            let close_brace = Some(Node::new(
                self.skip(Self::HIDDEN),
                self.expect(TokenKind::CloseBrace)?,
                Vec::new(),
            ));

            (close_brace, self.skip(Self::HIDDEN))
        } else {
            // Implicit close brace.
            let after = self.skip(Self::HIDDEN);
            self.expect(TokenKind::Eof)?;

            (None, after)
        };

        let node = Node::new(
            before,
            ast::Map {
                open_brace,
                members,
                close_brace,
            },
            after,
        );

        Ok(node)
    }

    fn parse_map_members(&mut self) -> ParseResult<Vec<Node<ast::MapMember>>> {
        let mut members = Vec::new();

        loop {
            let before = self.skip(Self::HIDDEN);

            let Some(key) = self.eat(Self::KEY) else {
                break;
            };

            let colon = Node::new(
                self.skip(Self::HIDDEN),
                self.expect(TokenKind::Colon)?,
                self.skip(Self::HIDDEN),
            );

            let value = self.expect_value()?;

            let mut after = self.skip(Self::HIDDEN_LINE);

            let comma = if let Some(comma) = self.eat(&[TokenKind::Comma]) {
                // Explicit comma.
                let before = mem::take(&mut after);
                let node = Node::new(before, comma, self.skip(Self::HIDDEN));
                Some(node)
            } else if let Some(newline) = self.eat(&[TokenKind::NewLine]) {
                // Implicit comma.
                after.push(newline);
                None
            } else {
                // End of members.
                break;
            };

            after.extend(self.skip(Self::HIDDEN));

            let node = Node::new(
                before,
                ast::MapMember {
                    key,
                    colon,
                    value,
                    comma,
                },
                after,
            );
            members.push(node);
        }

        Ok(members)
    }

    fn parse_value(&mut self) -> ParseResult<Option<ast::Value>> {
        let map = self.parse_map()?.map(ast::Value::Map);
        if map.is_some() {
            return Ok(map);
        }

        let array = self.parse_array()?.map(ast::Value::Array);
        if array.is_some() {
            return Ok(array);
        }

        let value = self.eat(Self::VALUE).map(ast::Value::Value);
        if value.is_some() {
            return Ok(value);
        }

        Ok(None)
    }

    fn parse_map(&mut self) -> ParseResult<Option<ast::Map>> {
        let Some(open_brace) = self.eat(&[TokenKind::OpenBrace]) else {
            return Ok(None)
        };

        let open_brace = Some(Node::new(
            Vec::new(),
            open_brace,
            self.skip(Self::HIDDEN_LINE),
        ));

        let members = self.parse_map_members()?;

        let close_brace = Some(Node::new(
            self.skip(Self::HIDDEN),
            self.expect(TokenKind::CloseBrace)?,
            Vec::new(),
        ));

        let map = ast::Map {
            open_brace,
            members,
            close_brace,
        };

        Ok(Some(map))
    }

    fn parse_array(&mut self) -> ParseResult<Option<ast::Array>> {
        let open_bracket = self.eat(&[TokenKind::OpenBracket]);

        let Some(open_bracket) = open_bracket else {
            return Ok(None);
        };

        let open_bracket = Node::new(Vec::new(), open_bracket, self.skip(Self::HIDDEN_LINE));

        let mut members = Vec::new();

        let mut before;
        loop {
            // if there's no value, this should become part of the close bracket's `before`.
            before = self.skip(Self::HIDDEN);

            let Some(value) = self.parse_value()? else {
                break;
            };

            let mut after = self.skip(Self::HIDDEN_LINE);

            let comma = if let Some(comma) = self.eat(&[TokenKind::Comma]) {
                // Explicit comma.
                let before = mem::take(&mut after);
                let node = Node::new(before, comma, self.skip(Self::HIDDEN));
                Some(node)
            } else if let Some(newline) = self.eat(&[TokenKind::NewLine]) {
                // Implicit comma.
                after.push(newline);
                None
            } else {
                // End of members.
                break;
            };

            let node = Node::new(before, ast::ArrayMember { value, comma }, after);
            members.push(node);
        }

        before.append(&mut self.skip(Self::HIDDEN));

        let close_bracket = Node::new(before, self.expect(TokenKind::CloseBracket)?, Vec::new());

        let array = ast::Array {
            open_bracket,
            members,
            close_bracket,
        };

        Ok(Some(array))
    }

    fn expect_value(&mut self) -> ParseResult<ast::Value> {
        if let Some(value) = self.parse_value()? {
            return Ok(value);
        }

        // This iterator returns an EOF token at the end (not `None`), so we can expect it.
        let (cursor, token) = self.tokens.next().expect("expected token");

        Err(ParseError {
            cursor,
            expected: String::from("value"),
            got: token.kind,
        })
    }

    #[must_use]
    fn eat(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        let Some((_, next)) = self.tokens.peek() else {
            return None;
        };

        if kinds.contains(&next.kind) {
            let (_, token) = self.tokens.next().expect("expected token");
            Some(token)
        } else {
            None
        }
    }

    #[must_use]
    fn skip(&mut self, kinds: &[TokenKind]) -> Vec<Token> {
        iter::from_fn(|| self.eat(kinds)).collect()
    }

    fn expect(&mut self, kind: TokenKind) -> ParseResult<Token> {
        // This iterator returns an EOF token at the end (not `None`), so we can expect it.
        let (cursor, token) = self.tokens.next().expect("expected token");

        if token.kind == kind {
            Ok(token)
        } else {
            Err(ParseError {
                cursor,
                expected: kind.to_string(),
                got: token.kind,
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParseError {
    cursor: Cursor,
    expected: String,
    got: TokenKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            expected,
            got,
            cursor: Cursor { line, column, .. },
        } = self;
        write!(f, "{line}:{column}: expected {expected}, got {got}",)
    }
}

impl Error for ParseError {}
