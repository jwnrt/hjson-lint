//! Collection of parsers for lexing a Hjson file into tokens.
//!
//! Each module contains a `parse` function which returns an `Option<Token>`.
//! The parsers match tokens at the start of the given string or return `None`.
//!
//! If parsers find partially valid tokens (e.g. a single `/` for a
//! comment, or `1.` for a number) they will return `None` and not an error.
//! This is because they could instead be valid unquoted strings in Hjson.

pub mod comment;
pub mod key;
pub mod keyword;
pub mod number;
pub mod symbol;
pub mod text;
pub mod whitespace;

use std::iter;

use crate::token::Token;

/// Return the next token from the given input with the given context.
pub fn token(input: &str, context: &Context) -> Token {
    let parsers = [
        whitespace::parse,
        comment::parse,
        keyword::parse,
        number::parse,
        symbol::parse,
    ]
    .into_iter();

    let text_parser = match context {
        Context::Key => key::parse,
        Context::Value => text::parse,
    };

    let mut parsers = parsers.chain(iter::once(text_parser));

    parsers.find_map(|p| p(input)).expect("no parser matched")
}

/// The context in which to perform lexical analysis.
///
/// The only context we need is whether or not to parse text as a key or value.
/// Keys are terminated at colons etc, while values are not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Context {
    Key,
    Value,
}
