//! provides utilities used throughout the project, such as error handling

use std::fmt::{Display, Formatter, Result as FmtResult};

/// represents the different kinds of errors that can occur
///
/// covers tokenization, parsing, and more.
#[derive(Debug)]
pub enum ErrorKind {
    InvalidKeyword(String),
    UnterminatedString,
    UnrecognizedToken,
    UnexpectedEnd,
}

/// represents a location, with line and column number
#[derive(Clone, Copy, Debug, Default)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

/// represents an error, containing file, line, and column information
#[derive(Debug)]
pub struct Error<'a> {
    pub loc: Location,
    pub file: &'a str,
    pub kind: ErrorKind,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "[{}:{}:{}] {}",
            self.file,
            self.loc.line,
            self.loc.col,
            match &self.kind {
                ErrorKind::InvalidKeyword(kw) => format!("encountered invalid keyword {}", kw),
                ErrorKind::UnterminatedString =>
                    "encountered unterminated string during tokenization".into(),
                ErrorKind::UnrecognizedToken =>
                    "encountered unrecognized token during tokenization".into(),
                ErrorKind::UnexpectedEnd => "encountered unexpected end-of-file".into(),
            }
        )
    }
}

impl<'a> std::error::Error for Error<'a> {}

/// type alias for a result using our error type
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

/// a simple macro for reporting an error message and exiting with a failure
#[macro_export]
macro_rules! err {
    ($msg:ident) => {{
        eprintln!("{}", $msg);
        std::process::exit(1);
    }};
    ($msg:expr) => {{
        eprintln!($msg);
        std::process::exit(1)
    }};
    ($msg:expr, $idx:ident) => {{
        eprintln!($msg, $idx);
        std::process::exit(1)
    }};
}
