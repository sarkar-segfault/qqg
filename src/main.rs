mod args;
mod token;
mod utils;

use std::fs::read_to_string;

fn main() {
    let info = args::parse();
    println!(
        "{:?}",
        token::ize(
            read_to_string(&info.file).unwrap_or_else(|e| {
                err!(e);
            }),
            info.file
        )
        .unwrap_or_else(|e| {
            err!(e);
        })
    )
}
