use crate::{
    fatal,
    utils::{Color, color},
};

fn help(prog: &str) -> ! {
    println!(
        "{}",
        color(
            Color::Yellow,
            &(env!("CARGO_PKG_NAME").to_owned() + " " + env!("CARGO_PKG_VERSION"))
        ),
    );
    println!(
        "created by {}, licensed under {}",
        color(Color::Yellow, env!("CARGO_PKG_AUTHORS")),
        color(Color::Yellow, env!("CARGO_PKG_LICENSE"))
    );
    println!(env!("CARGO_PKG_DESCRIPTION"));

    let input = color(Color::Grey, "<input.qq>");

    println!("\n{}", color(Color::Yellow, prog));
    println!(
        "    {}\n\tprints this message and exits",
        color(Color::Yellow, "help")
    );
    println!(
        "    {} {}\n\ttokenizes the provided file",
        color(Color::Yellow, "token"),
        input
    );
    println!(
        "    {} {}\n\ttokenizes and parses the provided file",
        color(Color::Yellow, "parse"),
        input
    );
    println!(
        "    {} {}\n\ttokenizes, parses and starts the provided file",
        color(Color::Yellow, "start"),
        input
    );

    std::process::exit(0);
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Parse,
    Token,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    pub cmd: Command,
    pub file: String,
}

fn get_filename(args: &mut impl Iterator<Item = String>) -> String {
    if let Some(file) = args.next() {
        if args.next().is_some() {
            fatal!(
                "{}",
                color(Color::Red, "encountered too many inputs; expected one")
            );
        }

        file
    } else {
        fatal!("{}", color(Color::Red, "expected input file"));
    }
}

pub fn parse(mut args: impl Iterator<Item = String>) -> Info {
    let prog = args
        .next()
        .unwrap_or_else(|| fatal!("{}", color(Color::Red, "expected program name")));
    let cmd = args
        .next()
        .unwrap_or_else(|| fatal!("{}", color(Color::Red, "expected subcommand")));

    if cmd == "help" {
        help(&prog);
    } else if cmd == "token" {
        Info {
            cmd: Command::Token,
            file: get_filename(&mut args),
        }
    } else if cmd == "parse" {
        Info {
            cmd: Command::Parse,
            file: get_filename(&mut args),
        }
    } else if cmd == "start" {
        Info {
            cmd: Command::Start,
            file: get_filename(&mut args),
        }
    } else {
        fatal!(
            "{}",
            color(Color::Red, "encountered unrecognized subcommand")
        );
    }
}
