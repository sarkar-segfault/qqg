mod args;
mod styles;
mod token;

use std::fs::read_to_string;
use std::process::exit;

fn main() {
    let info = args::parse();
    println!(
        "{:?}",
        token::ize(
            read_to_string(&info.file).unwrap_or_else(|e| {
                eprintln!("1: {}", e);
                exit(1);
            }),
            info.file
        )
        .unwrap_or_else(|e| {
            eprintln!("2: {}", e);
            exit(1);
        })
    )
}
