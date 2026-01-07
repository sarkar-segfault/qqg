use std::io::IsTerminal;

pub enum Color {
    Yellow,
    Green,
    Grey,
    Red,
}

pub fn color(kind: Color, text: &str) -> String {
    if !std::io::stdout().is_terminal() || std::env::var_os("NO_COLOR").is_some() {
        return text.to_string();
    }

    format!(
        "{}{}\x1b[0m",
        match kind {
            Color::Yellow => "\x1b[33m",
            Color::Green => "\x1b[32m",
            Color::Grey => "\x1b[90m",
            Color::Red => "\x1b[31m",
        },
        text
    )
}

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
    let suffix = color(
        Color::Grey,
        &("(during ".to_owned() + if parse { "parsing" } else { "tokenization" } + ")"),
    );

    let lbrack = color(Color::Grey, "[");
    let rbrack = color(Color::Grey, "]");
    let colon = color(Color::Grey, ":");
    let dots = color(Color::Grey, "..");

    let file = color(Color::Yellow, file);
    let msg = color(Color::Red, msg);

    let begin_line = color(Color::Yellow, &begin.line.to_string());
    let begin_col = color(Color::Yellow, &begin.col.to_string());
    let end_line = color(Color::Yellow, &end.line.to_string());
    let end_col = color(Color::Yellow, &end.col.to_string());

    if begin.line == end.line {
        if begin.col == end.col {
            fatal!(
                "{}{}{}{}{}{}{} {} {}",
                lbrack,
                file,
                colon,
                begin_line,
                colon,
                begin_col,
                rbrack,
                msg,
                suffix,
            );
        }
        fatal!(
            "{}{}{}{} {}{}{}{} {} {}",
            lbrack,
            file,
            colon,
            begin_line,
            begin_col,
            dots,
            end_col,
            rbrack,
            msg,
            suffix,
        );
    }
    fatal!(
        "{}{} {}{}{}{}{}{}{}{} {} {}",
        lbrack,
        file,
        begin_line,
        colon,
        begin_col,
        dots,
        end_line,
        colon,
        end_col,
        rbrack,
        msg,
        suffix,
    );
}

#[macro_export]
macro_rules! token_error {
    ($begin:expr, $end:expr, $msg:expr, $file:expr) => {{
        $crate::utils::error($begin, $end, $msg, $file, false);
    }};
}

#[macro_export]
macro_rules! parse_error {
    ($begin:expr, $end:expr, $msg:expr, $file: expr) => {{
        $crate::utils::error($begin, $end, $msg, $file, true);
    }};
}
