mod args;
mod ast;
mod run;
mod token;
mod utils;

fn main() {
    let info = args::parse();
    let text = &std::fs::read_to_string(&info.file)
        .unwrap_or_else(|e| fatal!("failed to open file: {}", e));

    match info.cmd {
        args::Command::Token => println!("{:#?}", token::ize(&info.file, text)),
        args::Command::Parse => println!(
            "{:#?}",
            ast::ify(&mut token::ize(&info.file, text), &info.file)
        ),
        args::Command::Start => run::start(ast::ify(&mut token::ize(&info.file, text), &info.file)),
    }
}
