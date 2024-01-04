mod lexer;
mod parser;
mod token;
mod tree;

use parser::Parser;

pub fn parse(input: &str) {
    let events = Parser::parse(input);

    for event in events {
        println!("{event:#?}");
    }
}
