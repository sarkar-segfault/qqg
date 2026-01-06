use crate::fatal;

fn help(prog: &str) -> ! {
    println!(concat!(
        env!("CARGO_PKG_NAME"),
        ", version ",
        env!("CARGO_PKG_VERSION")
    ));
    println!(env!("CARGO_PKG_DESCRIPTION"));
    println!(concat!(
        "created by ",
        env!("CARGO_PKG_AUTHORS"),
        ", licensed under ",
        env!("CARGO_PKG_LICENSE")
    ));

    println!("\n{}", prog);
    println!("    help\n\tprints this message and exits");
    println!("    token <input.qq>\n\ttokenizes the provided file");
    println!("    parse <input.qq>\n\ttokenizes and parses the provided file");
    println!("    start <input.qq>\n\ttokenizes, parses and starts the provided file");

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
        fatal!("expected input file");
    }
}

pub fn parse() -> Info {
    let mut args = std::env::args();
    let prog = args
        .next()
        .unwrap_or_else(|| fatal!("expected program name"));
    let cmd = args.next().unwrap_or_else(|| fatal!("expected subcommand"));

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
        fatal!("encountered unrecognized subcommand");
    }
}
