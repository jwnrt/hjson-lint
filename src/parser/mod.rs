use std::error::Error;
use std::fmt::{self, Display};
use std::iter::Peekable;

use crate::lexer::{Cursor, Token, TokenKind, Tokens};

type ParseResult<T> = Result<T, ParseError>;

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

    pub fn parse(input: &'a str) -> ParseResult<()> {
        let tokens = Tokens::parse(input).peekable();
        let mut parser = Self { tokens };

        parser.parse_root()
    }

    fn parse_root(&mut self) -> ParseResult<()> {
        self.skip(Self::HIDDEN);
        let opening_brace = self.eat(&[TokenKind::OpenBrace]);

        self.parse_map_members()?;

        if opening_brace {
            // Explicit close brace.
            self.expect(TokenKind::CloseBrace)?;
        } else {
            // Implicit close brace.
            self.expect(TokenKind::Eof)?;
        }

        Ok(())
    }

    fn parse_map_members(&mut self) -> ParseResult<()> {
        loop {
            self.skip(Self::HIDDEN);
            if !self.eat(Self::KEY) {
                break;
            }

            self.skip(Self::HIDDEN);
            self.expect(TokenKind::Colon)?;

            self.skip(Self::HIDDEN);
            self.expect_value()?;

            self.skip(Self::HIDDEN_LINE);
            if !self.eat(&[TokenKind::Comma, TokenKind::NewLine]) {
                break;
            }
        }

        Ok(())
    }

    fn parse_value(&mut self) -> ParseResult<bool> {
        self.skip(Self::HIDDEN);

        if self.parse_map()? {
            return Ok(true);
        }

        if self.parse_array()? {
            return Ok(true);
        }

        if self.eat(Self::VALUE) {
            return Ok(true);
        }

        Ok(false)
    }

    fn parse_map(&mut self) -> ParseResult<bool> {
        self.skip(Self::HIDDEN);

        if !self.eat(&[TokenKind::OpenBrace]) {
            return Ok(false);
        }

        self.skip(Self::HIDDEN);
        self.parse_map_members()?;

        self.expect(TokenKind::CloseBrace)?;

        Ok(true)
    }

    fn parse_array(&mut self) -> ParseResult<bool> {
        self.skip(Self::HIDDEN);
        if !self.eat(&[TokenKind::OpenBracket]) {
            return Ok(false);
        }

        loop {
            self.skip(Self::HIDDEN);
            self.parse_value()?;

            self.skip(Self::HIDDEN_LINE);

            if !self.eat(&[TokenKind::Comma, TokenKind::NewLine]) {
                break;
            }
        }

        self.skip(Self::HIDDEN);
        self.expect(TokenKind::CloseBracket)?;

        Ok(true)
    }

    fn expect_value(&mut self) -> ParseResult<bool> {
        self.skip(Self::HIDDEN);
        if self.parse_value()? {
            return Ok(true);
        }

        // This iterator returns an EOF token at the end (not `None`), so we can expect it.
        let (cursor, token) = self.tokens.next().expect("expected token");

        Err(ParseError {
            cursor,
            expected: String::from("value"),
            got: token.kind,
        })
    }

    fn eat(&mut self, kinds: &[TokenKind]) -> bool {
        let Some((_, next)) = self.tokens.peek() else {
            return false;
        };

        if kinds.contains(&next.kind) {
            let (_, token) = self.tokens.next().expect("expected token");
            true
        } else {
            false
        }
    }

    fn skip(&mut self, kinds: &[TokenKind]) {
        while self.eat(kinds) {}
    }

    fn expect(&mut self, kind: TokenKind) -> ParseResult<()> {
        // This iterator returns an EOF token at the end (not `None`), so we can expect it.
        let (cursor, token) = self.tokens.next().expect("expected token");

        if token.kind == kind {
            Ok(())
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
