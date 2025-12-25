use crate::styles::link;
use std::process::exit;

#[derive(Default)]
pub struct Info {
    pub file: String,
    pub verbose: bool,
    pub output: String,
    pub styles: Vec<String>,
    pub scripts: Vec<String>,
}

#[inline(always)]
fn version() {
    println!(
        "qqg (quick quiz generator) version {}",
        env!("CARGO_PKG_VERSION")
    )
}

#[inline(always)]
fn help() {
    version();
    println!("generate web and console-based quizzes blazingly fast");
    println!(
        "written in {} by {}, licensed under the GPL v3.0",
        link("rust-lang", "https://github.com/rust-lang"),
        link("sarkar-segfault", "https://github.com/sarkar-segfault")
    );
    println!(
        "for more information, refer to {}\n",
        link(
            "the qqg github repository",
            "https://github.com/sarkar-segfault/qqg.git"
        )
    );

    println!(
        "usage: qqg [--help|--version|--verbose|--output <output.html>|--style <styles.css>|--script <script.js>] <input.qq>"
    );
    println!("--help: prints this message and exits");
    println!("--version: prints the version and exits");
    println!("--verbose: toggle extensive log messages");
    println!("--output <output.html>: specify the output file (for web)");
    println!("--style <style.css>: specify stylesheets (for web)");
    println!("--script <script.js>: specify scripts (for web)");
    println!("<file>: the input quiz file")
}

pub fn parse() -> Info {
    let mut args = std::env::args();
    let mut info = Info::default();
    args.next();

    while let Some(arg) = args.next() {
        if arg == "--version" {
            version();
            exit(0);
        } else if arg == "--help" {
            help();
            exit(0);
        } else if arg == "--verbose" {
            info.verbose = !info.verbose;
        } else if arg == "--output" {
            if let Some(output) = args.next() {
                info.output = output;
            } else {
                eprintln!("expected output file after --output");
            }
        } else if arg == "--style" {
            if let Some(style) = args.next() {
                info.styles.push(style);
            } else {
                eprintln!("expected stylesheet after --style");
            }
        } else if arg == "--script" {
            if let Some(script) = args.next() {
                info.scripts.push(script);
            } else {
                eprintln!("expected script after --script");
            }
        } else {
            info.file = arg;
        }
    }

    if info.file.is_empty() {
        eprintln!("expected input file to be provided");
        exit(1);
    }

    info
}
