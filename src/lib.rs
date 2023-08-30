//! Hjson Linter
//!
//! This library contains modules for [lexing][lexer], [parsing][parser], and [linting][linter]
//! [Hjson] documents.
//!
//! Currently, the lexer and linter are tested but not against a large corpus of files. The
//! parser has no automated testing except indirectly through the linter.
//!
//! The parser generates a format-preserving AST (probably a concrete syntax tree actually)
//! so that the linter can check whitespace, comments, etc.
//!
//! [Hjson]: https://hjson.github.io/

pub mod lexer;
pub mod linter;
pub mod parser;
