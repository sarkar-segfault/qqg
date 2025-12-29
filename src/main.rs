mod args;
mod ast;
mod token;
mod utils;

fn main() {
    let _file = &args::parse().file;
}
