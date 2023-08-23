use std::io;

use hjson_lint::parser::Parser;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("failed to read stdin");

    let root = Parser::parse(&input).expect("failed to parse");

    println!("{root:#?}");
}
