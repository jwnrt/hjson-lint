use std::error::Error;
use std::fmt::{self, Display};
use std::iter;
use std::iter::Peekable;
use std::mem;

use crate::lexer::{Cursor, Span, TokenKind, Tokens};

type ParseResult<T> = Result<T, ParseError>;

pub mod ast;

use ast::Node;

pub struct Parser<'a> {
    tokens: Peekable<Tokens<'a>>,
}

impl<'a> Parser<'a> {
    const HIDDEN: &'static [TokenKind] = &[
        TokenKind::Whitespace,
        TokenKind::NewLine,
        TokenKind::LineComment,
        TokenKind::HashComment,
        TokenKind::BlockComment,
    ];

    const HIDDEN_LINE: &'static [TokenKind] = &[
        TokenKind::Whitespace,
        TokenKind::LineComment,
        TokenKind::HashComment,
        TokenKind::BlockComment,
    ];

    const KEY: &'static [TokenKind] = &[
        TokenKind::TextSingle,
        TokenKind::TextDouble,
        TokenKind::TextUnquoted,
    ];

    const VALUE: &'static [TokenKind] = &[
        TokenKind::Boolean,
        TokenKind::Integer,
        TokenKind::Float,
        TokenKind::TextSingle,
        TokenKind::TextDouble,
        TokenKind::TextMulti,
        TokenKind::TextUnquoted,
        TokenKind::Null,
    ];

    pub fn parse(input: &'a str) -> ParseResult<ast::Map> {
        let tokens = Tokens::parse(input).peekable();
        let mut parser = Self { tokens };

        parser.parse_root()
    }

    fn parse_root(&mut self) -> ParseResult<ast::Map> {
        let open_brace = Node::new(
            self.skip(Self::HIDDEN),
            self.eat(&[TokenKind::OpenBrace]),
            self.skip(Self::HIDDEN_LINE),
        );

        let members = self.parse_map_members()?;

        let close_brace = Node::new(
            self.skip(Self::HIDDEN),
            open_brace
                .inner
                .as_ref()
                .map(|_| self.expect(TokenKind::CloseBrace))
                .transpose()?,
            self.skip(Self::HIDDEN),
        );

        let root = ast::Map {
            open_brace,
            members,
            close_brace,
        };

        Ok(root)
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

            let mut comma_before = self.skip(Self::HIDDEN_LINE);
            let comma = self.eat(&[TokenKind::Comma]);
            let mut comma_after = match &comma {
                Some(_) => self.skip(Self::HIDDEN_LINE),
                None => mem::take(&mut comma_before),
            };
            if let Some(term) = self.eat(&[TokenKind::NewLine, TokenKind::Eof]) {
                comma_after.push(term);
            }
            let comma = Node::new(comma_before, comma, comma_after);

            let node = Node::new(
                before,
                ast::MapMember {
                    key,
                    colon,
                    value,
                    comma,
                },
                self.skip(Self::HIDDEN),
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
            return Ok(None);
        };

        let open_brace = Node::new(Vec::new(), Some(open_brace), self.skip(Self::HIDDEN_LINE));

        let members = self.parse_map_members()?;

        let close_brace = Node::new(
            self.skip(Self::HIDDEN),
            Some(self.expect(TokenKind::CloseBrace)?),
            Vec::new(),
        );

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
            // if there's no value, this space will become part of the close bracket's `before`.
            before = self.skip(Self::HIDDEN);

            let Some(value) = self.parse_value()? else {
                break;
            };

            let mut comma_before = self.skip(Self::HIDDEN_LINE);
            let comma = self.eat(&[TokenKind::Comma]);
            let mut comma_after = match &comma {
                Some(_) => self.skip(Self::HIDDEN_LINE),
                None => mem::take(&mut comma_before),
            };
            if let Some(term) = self.eat(&[TokenKind::NewLine, TokenKind::Eof]) {
                comma_after.push(term);
            }
            let comma = Node::new(comma_before, comma, comma_after);

            let node = Node::new(before, ast::ArrayMember { value, comma }, Vec::new());
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
        let next = self.tokens.next().expect("expected token");

        Err(ParseError {
            expected: String::from("value"),
            got: next,
        })
    }

    #[must_use]
    fn eat(&mut self, kinds: &[TokenKind]) -> Option<Span> {
        let Some(next) = self.tokens.peek() else {
            return None;
        };

        if kinds.contains(&next.kind) {
            // If EOF, give the peeked token without taking it off the iterator.
            if next.kind == TokenKind::Eof {
                Some(next.clone())
            } else {
                Some(self.tokens.next().expect("expected token"))
            }
        } else {
            None
        }
    }

    #[must_use]
    fn skip(&mut self, kinds: &[TokenKind]) -> Vec<Span> {
        iter::from_fn(|| self.eat(kinds)).collect()
    }

    fn expect(&mut self, kind: TokenKind) -> ParseResult<Span> {
        let next = self.tokens.next().expect("expected token");

        if next.kind == kind {
            Ok(next)
        } else {
            Err(ParseError {
                expected: kind.to_string(),
                got: next,
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParseError {
    got: Span,
    expected: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            expected,
            got:
                Span {
                    start: Cursor { line, column, .. },
                    kind,
                    ..
                },
        } = self;
        write!(f, "{line}:{column}: expected {expected}, got {kind}")
    }
}

impl Error for ParseError {}
