mod args;
mod ast;
mod parser;
mod token;
mod utils;

use std::fs::read_to_string;

fn main() {
    let file = &args::parse().file;
}
