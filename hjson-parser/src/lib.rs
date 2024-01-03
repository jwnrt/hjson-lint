mod lexer;
mod token;

use token::Token;

/// Run a line of text through all the lexers and return the first match, if
/// any.
///
/// This function is not currently useful because it doesn't take into
/// account key/value context. If some text contains a `:`, then it will be
/// matched by the `key` lexer first and terminate there.
pub fn parse_any(input: &str) -> Option<Token> {
    let parsers = [
        lexer::whitespace::parse,
        lexer::comment::parse,
        lexer::keyword::parse,
        lexer::number::parse,
        lexer::symbol::parse,
        lexer::key::parse,
        lexer::text::parse,
    ];

    parsers.into_iter().find_map(|p| p(input))
}
