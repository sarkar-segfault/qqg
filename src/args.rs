use crate::{
    fatal,
    utils::{Color, color},
};

fn help(prog: &str) -> ! {
    println!(
        "{}, version {}",
        color(Color::Green, env!("CARGO_PKG_NAME")),
        color(Color::Green, env!("CARGO_PKG_VERSION"))
    );
    println!(
        "generate {}, console-based quizzes {}",
        color(Color::Green, "interactive"),
        color(Color::Green, "blazingly fast")
    );
    println!(
        "created by {}, licensed under {}",
        color(Color::Green, env!("CARGO_PKG_AUTHORS")),
        color(Color::Green, env!("CARGO_PKG_LICENSE"))
    );

    let input = color(Color::Grey, "<input.qq>");

    println!("\n{}", color(Color::Green, prog));
    println!(
        "    {}\n\tprints this message and exits",
        color(Color::Green, "help")
    );
    println!(
        "    {} {}\n\ttokenizes the provided file",
        color(Color::Green, "token"),
        input
    );
    println!(
        "    {} {}\n\ttokenizes and parses the provided file",
        color(Color::Green, "parse"),
        input
    );
    println!(
        "    {} {}\n\ttokenizes, parses and starts the provided file",
        color(Color::Green, "start"),
        input
    );

    std::process::exit(0);
}

pub enum Command {
    Parse,
    Token,
    Start,
}

pub struct Info {
    pub cmd: Command,
    pub file: String,
}

fn get_filename(args: &mut std::env::Args) -> String {
    if let Some(file) = args.next() {
        file
    } else {
        fatal!("{}", color(Color::Red, "expected input file"));
    }
}

pub fn parse() -> Info {
    let mut args = std::env::args();
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
