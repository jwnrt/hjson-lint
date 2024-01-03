use std::io;

use hjson_lint::linter::{Config, Linter};

fn main() {
    let input = io::read_to_string(io::stdin()).expect("failed to read stdin");

    let lints = Linter::lint(Config::strict(), &input).expect("failed to lint");

    println!("{lints:#?}");
}
