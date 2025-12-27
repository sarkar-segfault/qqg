//! provides functions and structs related to command-line argument parsing

use crate::err;
use std::process::exit;

/// represents arguments parsed from the command-line
#[derive(Default)]
pub struct Info {
    pub file: String,
    pub styles: Vec<String>,
    pub scripts: Vec<String>,
}

/// prints the version line
fn version() {
    println!(
        "{}, version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )
}

/// prints help information
fn help() {
    version();
    println!(env!("CARGO_PKG_DESCRIPTION"));
    println!(
        "created by {} and licensed under {}",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_LICENSE")
    );
    println!(
        "home at {}\ndocs at https://docs.rs/quick-quiz-generator\n",
        env!("CARGO_PKG_HOMEPAGE")
    );

    println!("usage: qqg [-v|-h|-i <input.qq>|-c <styles.css>|-j <script.js>]");
    println!("\t-v: prints the version and exits");
    println!("\t-h: prints this message and exits");
    println!("\t-i <input.qq>: specify the input quiz file");
    println!("\t-j <script.js>: specify a script file (web)");
    println!("\t-c <styles.css>: specify a stylesheet (web)");
}

/// parses command-line arguments into a [`Info`]
///
/// # exits with success
/// if `-v` or `-h` is detected
///
/// # exits with failure
/// if a filename is missing after `-i`, `-c`, or `-j`; or if `-i` is not provided
pub fn parse() -> Info {
    let mut args = std::env::args();
    let mut info = Info::default();
    let mut index = 1;
    args.next();

    while let Some(arg) = args.next() {
        if arg == "-v" {
            version();
            exit(0);
        } else if arg == "-h" {
            help();
            exit(0);
        } else if arg == "-i" {
            if let Some(input) = args.next() {
                info.file = input;
            } else {
                err!("expected input file after -i");
            }
        } else if arg == "-c" {
            if let Some(style) = args.next() {
                info.styles.push(style);
            } else {
                err!("expected stylesheet after -c");
            }
        } else if arg == "-j" {
            if let Some(script) = args.next() {
                info.scripts.push(script);
            } else {
                err!("expected script file after -j");
            }
        } else {
            err!("got malformed argument at {}", index);
        }

        index += 1;
    }

    if info.file.is_empty() {
        err!("expected input file to be provided");
    }

    info
}
