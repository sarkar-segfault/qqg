mod args;
mod ast;
mod token;
mod utils;

fn main() {
    let file = &args::parse().file;
    println!(
        "{:#?}",
        ast::ify(
            &mut token::ize(
                std::fs::read_to_string(file).unwrap_or_else(|e| err!(e)),
                file
            )
            .unwrap_or_else(|e| err!(e)),
            file
        )
        .unwrap_or_else(|e| err!(e))
    )
}
