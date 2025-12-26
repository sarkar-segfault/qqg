use crate::{err, link};
use std::process::exit;

#[derive(Default)]
pub struct Info {
    pub file: String,
    pub styles: Vec<String>,
    pub scripts: Vec<String>,
}

fn version() {
    println!(
        "qqg (quick quiz generator) version {}",
        env!("CARGO_PKG_VERSION")
    )
}

fn help() {
    version();
    println!("generate web and console-based quizzes blazingly fast");
    println!(
        "written in {} by {}, licensed under {}",
        link!("rust-lang", "https://github.com/rust-lang"),
        link!("sarkar-segfault", "https://github.com/sarkar-segfault"),
        link!("gpl v3.0", "https://www.gnu.org/licenses/gpl-3.0.html")
    );
    println!(
        "for more information, refer to the {}\n",
        link!(
            "qqg github repository",
            "https://github.com/sarkar-segfault/qqg.git"
        )
    );

    println!("usage: qqg [-v|-h|-i <input.qq>|-c <styles.css>|-j <script.js>]");
    println!("\t-v: prints the version and exits");
    println!("\t-h: prints this message and exits");
    println!("\t-i <input.qq>: specify the input quiz file");
    println!("\t-j <script.js>: specify a script file (web)");
    println!("\t-c <styles.css>: specify a stylesheet (web)");
}

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
