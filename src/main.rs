mod args;
mod token;
mod utils;

use args::Command;

fn main() {
    let info = args::parse();
    let text = &std::fs::read_to_string(&info.file)
        .unwrap_or_else(|e| fatal!("failed to open file: {}", e));

    match info.cmd {
        Command::Token => println!("{:#?}", token::ize(&info.file, text)),
        _ => todo!("implement later"),
    }
}
