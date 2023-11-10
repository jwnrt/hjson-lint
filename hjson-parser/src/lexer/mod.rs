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
