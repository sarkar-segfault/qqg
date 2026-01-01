use crate::utils::fatal;

#[derive(Default)]
pub struct Info {
    pub files: Vec<String>,
}

impl Info {
    fn help(prog: String) -> ! {
        println!(
            "{} {}\n{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION")
        );
        println!(
            "created by {}, licensed under {}",
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_LICENSE")
        );
        println!("usage: {} [-h|-i <..files>]", prog);

        std::process::exit(0);
    }

    pub fn new() -> Info {
        let mut args = std::env::args();
        let prog = args
            .next()
            .unwrap_or_else(|| fatal("expected program name as first argument"));

        let mut index: usize = 0;
        let mut info = Info::default();

        while let Some(arg) = args.next() {
            if arg == "-h" {
                Self::help(prog);
            } else if arg == "-i" {
                while let Some(file) = args.next() {
                    info.files.push(file);
                }
            } else {
                fatal(&format!("encountered invalid argument at {}", index));
            }

            index += 1;
        }

        if info.files.is_empty() {
            fatal("expected input filenames based on invokation");
        }

        info
    }
}
