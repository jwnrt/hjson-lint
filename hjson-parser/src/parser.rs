//! Parser for Hjson files.
//!
//! The architecture for this parser is based on this excellent article:
//! <https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html>.
//! The only small difference is that we do lexical analysis on-the-fly because
//! lexing changes depending on the parser's context.
//!
//! We keep track of the current [`Context`] using a stack. The only important
//! context we need is whether text should be parsed as a key or value.
//!
//! I'm hoping to make this parser error resilient so that we can lint files
//! that are incorrectly specified. I don't intend to make a full-blown LSP
//! server or anything just for Hjson.

use crate::lexer::{self, Context};
use crate::token::{Token, TokenKind};
use crate::tree::TreeKind;

/// Hjson parser which generates [`Event`]s from an input string.
///
/// These events describe a tree structure, but are collected linearly during
/// parsing for simplicity.
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    input: &'a str,
    current: Token,
    context: Vec<Context>,
    events: Vec<Event>,
}

/// Parsing events.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// Open a new tree with the given `kind`.
    Open { kind: TreeKind },
    /// Close the most recently opened tree.
    Close,
    /// Advance through the input, taking this `token`.
    Advance { token: Token },
}

/// A mark for a position in the event stream.
///
/// Trees are opened with the [`TreeKind::ErrorTree`] kind, which is updated to
/// the correct `TreeKind` when they are successfully parsed. These marks allow
/// us to update `Open` events from back in the event stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MarkOpened {
    /// Index in the event stream of the [`Event::Open`] event.
    index: usize,
}

impl Parser<'_> {
    /// Parse the given Hjson file, returning a stream of events.
    pub fn parse(input: &str) -> Vec<Event> {
        let mut parser = Parser {
            input,
            current: lexer::token(input, &Context::Key),
            context: Vec::from([Context::Key]),
            events: Vec::new(),
        };

        file(&mut parser);

        parser.events
    }

    /// Open a new tree here.
    ///
    /// This tree has the [`TreeKind::ErrorTree`] kind until it is parsed
    /// successfully and updated through passing the returned [`MarkOpened`] to
    /// [`close`].
    #[must_use]
    fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: TreeKind::ErrorTree,
        });
        mark
    }

    /// Close the tree opened at the given `mark`, giving it a specific `kind`.
    fn close(&mut self, mark: MarkOpened, kind: TreeKind) {
        self.events[mark.index] = Event::Open { kind };
        self.events.push(Event::Close);
    }

    /// Advance the parser to the next token.
    fn advance(&mut self) {
        let token = self.current;

        self.input = &self.input[token.len..];
        self.events.push(Event::Advance { token });

        self.relex_token();
    }

    /// Checks whether the parser is at the end of the file.
    #[must_use]
    fn eof(&self) -> bool {
        self.input.is_empty()
    }

    /// Checks whether the parser is at a certain kind of token.
    #[must_use]
    fn at(&self, kind: TokenKind) -> bool {
        self.current.kind == kind
    }

    /// Checks whether the parser is at any of the given kinds of token.
    #[must_use]
    fn at_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(&self.current.kind)
    }

    /// If the parser is at a certain kind of token, consume it and advance to
    /// the next one.
    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// If the parser is at any one of the given kinds of token, consume it and
    /// advance to the next one.
    fn eat_any(&mut self, kinds: &[TokenKind]) -> bool {
        if self.at_any(kinds) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consume all tokens matching any of the given kinds of token, advancing
    /// to the next token that does not match an of those kinds.
    fn eat_all(&mut self, kinds: &[TokenKind]) {
        while self.eat_any(kinds) {}
    }

    /// Expect some kind of token, consuming it if found, generating an error
    /// otherwise.
    ///
    /// The parser does _not_ advance if the token did not match.
    fn expect(&mut self, kind: TokenKind) {
        if self.eat(kind) {
            return;
        }

        // TODO: error reporting.
        eprintln!("expected {kind:?}, got {:?}", self.current.kind);
    }

    /// Expect some token matching one of the given kinds of token.
    ///
    /// The given `name` will be used for the expected token in errors.
    fn expect_some(&mut self, kinds: &[TokenKind], name: &str) {
        if self.eat_any(kinds) {
            return;
        }

        // TODO: error reporting.
        eprintln!("expected {name}");
    }

    /// Advance to the next token and generate an error.
    fn advance_with_error(&mut self, error: &str) {
        let mark = self.open();
        self.advance();
        self.close(mark, TreeKind::ErrorTree);

        //TODO: error reporting.
        eprintln!("{error}");
    }

    /// Re-lex (lexically analyze) the current token from the input.
    fn relex_token(&mut self) {
        self.current = lexer::token(self.input, self.context.last().unwrap());
    }

    /// Push a context to the parser's stack.
    fn push_context(&mut self, context: Context) {
        let old_context = *self.context.last().unwrap();
        self.context.push(context);

        if old_context != context {
            self.relex_token();
        }
    }

    /// Pop a context from the parser's stack.
    fn pop_context(&mut self) {
        let old_context = self.context.pop().unwrap();

        if old_context != *self.context.last().unwrap() {
            self.relex_token();
        }
    }
}

/// Token kinds representing decoration with no semantic significance.
const DECO: &[TokenKind; 4] = &[
    TokenKind::Whitespace,
    TokenKind::LineComment,
    TokenKind::HashComment,
    TokenKind::BlockComment,
];

/// Token kinds that separate other tokens but are usually not significant.
const SEPARATOR: &[TokenKind; 5] = &[
    TokenKind::Whitespace,
    TokenKind::LineComment,
    TokenKind::HashComment,
    TokenKind::BlockComment,
    TokenKind::NewLine,
];

/// Token kinds that can represent keys of a mapping.
const KEY: &[TokenKind; 3] = &[
    TokenKind::TextSingle,
    TokenKind::TextDouble,
    TokenKind::TextUnquoted,
];

/// Token kinds that can represent the start of a value.
const VALUE: &[TokenKind; 10] = &[
    TokenKind::Bool,
    TokenKind::Null,
    TokenKind::Integer,
    TokenKind::Float,
    TokenKind::TextSingle,
    TokenKind::TextDouble,
    TokenKind::TextMulti,
    TokenKind::TextUnquoted,
    // Maps and arrays are also values.
    TokenKind::LBrace,
    TokenKind::LBracket,
];

/// Parse a full Hjson file.
fn file(p: &mut Parser) {
    let mark = p.open();

    p.eat_all(SEPARATOR);

    if p.at(TokenKind::LBrace) {
        map(p, true);
    } else if p.at_any(KEY) {
        map(p, false)
    } else {
        p.advance_with_error("expected map")
    }

    p.close(mark, TreeKind::File);

    // TODO: expect EOF here?
}

/// Parse a full map, optionally requiring that it has surrounding braces.
fn map(p: &mut Parser, braces: bool) {
    let mark = p.open();

    if braces {
        p.expect(TokenKind::LBrace);
    }

    while !p.eof() {
        p.eat_all(SEPARATOR);

        if p.at(TokenKind::RBrace) {
            if braces {
                break;
            } else {
                p.advance_with_error("unexpected close brace");
                continue;
            }
        }

        if p.at_any(KEY) {
            mapping(p);
            p.eat_all(DECO);

            if !p.at(TokenKind::RBrace) && !p.eof() {
                p.expect_some(&[TokenKind::Comma, TokenKind::NewLine], "comma or new-line");
            }
        } else {
            p.advance_with_error("expected mapping");
        }
    }

    if braces {
        p.expect(TokenKind::RBrace);
    }

    p.close(mark, TreeKind::Map);
}

/// Parse a mapping (`key: value`).
fn mapping(p: &mut Parser) {
    p.push_context(Context::Key);

    let mark = p.open();

    p.expect_some(KEY, "key");
    p.eat_all(SEPARATOR);
    p.expect(TokenKind::Colon);
    p.eat_all(SEPARATOR);

    if p.at_any(VALUE) {
        value(p);
    } else {
        p.advance_with_error("expected value");
    }

    p.close(mark, TreeKind::Mapping);
    p.pop_context();
}

/// Parse an array.
fn array(p: &mut Parser) {
    let mark = p.open();

    p.expect(TokenKind::LBracket);

    while !p.eof() {
        p.eat_all(SEPARATOR);

        if p.at(TokenKind::RBracket) {
            break;
        }

        if p.at_any(VALUE) {
            value(p);
            p.eat_all(DECO);

            if !p.at(TokenKind::RBracket) && !p.eof() {
                p.expect_some(&[TokenKind::Comma, TokenKind::NewLine], "comma or new-line");
            }
        } else {
            p.advance_with_error("expected value");
        }
    }

    p.expect(TokenKind::RBracket);

    p.close(mark, TreeKind::Array);
}

/// Parse a value (text, number, Boolean, map, array, null).
fn value(p: &mut Parser) {
    p.push_context(Context::Value);

    if p.at(TokenKind::LBrace) {
        map(p, true);
    } else if p.at(TokenKind::LBracket) {
        array(p);
    } else {
        p.expect_some(VALUE, "value");
    }

    p.pop_context();
}
