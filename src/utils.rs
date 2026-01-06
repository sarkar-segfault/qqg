#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {{
        eprintln!($($arg)+);
        std::process::exit(0);
    }}
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, col: 1 }
    }
}

pub fn error(begin: Location, end: Location, msg: &str, file: &str, parse: bool) -> ! {
    let suffix = if parse { "parsing" } else { "tokenization" };

    if begin.line == end.line {
        if begin.col == end.col {
            fatal!(
                "[{}:{}:{}] {} (during {})",
                file,
                begin.line,
                begin.col,
                msg,
                suffix
            );
        }
        fatal!(
            "[{}:{} {}..{}] {} (during {})",
            file,
            begin.line,
            begin.col,
            end.col,
            msg,
            suffix
        );
    }
    fatal!(
        "[{} {}:{}..{}:{}] {} (during {})",
        file,
        begin.line,
        begin.col,
        end.line,
        end.col,
        msg,
        suffix
    );
}
