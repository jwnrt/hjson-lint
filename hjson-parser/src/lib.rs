mod lexer;
mod parser;
mod token;
mod tree;

use parser::Parser;
pub use tree::Tree;

pub fn parse(input: &str) -> Tree {
    let events = Parser::parse(input);
    Tree::build(events)
}
