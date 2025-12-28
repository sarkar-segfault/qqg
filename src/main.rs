mod args;
mod ast;
mod token;
mod utils;

use std::fs::read_to_string;

fn main() {
    let file = &args::parse().file;
    ast::ify(
        token::ize(read_to_string(file).unwrap_or_else(|e| err!(e)), file)
            .unwrap_or_else(|e| err!(e)),
        file,
    )
    .unwrap_or_else(|e| err!(e));
}
