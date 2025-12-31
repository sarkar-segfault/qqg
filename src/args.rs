use crate::err;
use std::io::{Write, stdin, stdout};
use std::process::exit;

fn version() {
    println!(
        "{}, version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}

fn help() {
    version();
    println!(env!("CARGO_PKG_DESCRIPTION"));
    println!(
        "created by {} and licensed under {}",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_LICENSE")
    );

    println!("usage: qqg [-v|-h] <input>");
    println!("\t-v: prints the version and exits");
    println!("\t-h: prints this message and exits");
    println!("\t<input>: specify the input quiz file");
}

pub fn parse() -> String {
    let mut args = std::env::args();
    let mut file = String::new();
    args.next();

    for arg in args {
        if arg == "-v" {
            version();
            exit(0);
        } else if arg == "-h" {
            help();
            exit(0);
        } else {
            file = arg;
        }
    }

    if file.is_empty() {
        print!("enter an input quiz file: ");
        stdout().flush().unwrap_or_else(|e| err!(e));
        stdin().read_line(&mut file).unwrap_or_else(|e| err!(e));
        file = file.trim().to_string();
    }

    file
}
