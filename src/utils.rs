//! provides utilities used throughout the project, such as error handling

use crate::token::TokenKind;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// represents the different kinds of errors that can occur
///
/// covers tokenization, parsing, and more.
#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken(String, String),
    InvalidKeyword(String),
    UnterminatedString,
    UnexpectedEnd,
    BadToken,
}

/// represents a location, with line and column number
#[derive(Clone, Copy, Debug, Default)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

/// represents an error, containing file, line, and column information
#[derive(Debug)]
pub struct Error {
    pub loc: Location,
    pub file: String,
    pub kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "[{}:{}:{}] {}",
            self.file,
            self.loc.line,
            self.loc.col,
            match &self.kind {
                ErrorKind::UnexpectedToken(expect, found) => format!(
                    "encountered unexpected token during tokenization\nexpected {:?}, found {:?}",
                    expect, found
                ),
                ErrorKind::InvalidKeyword(kw) =>
                    format!("encountered invalid keyword during tokenization: {}", kw),
                ErrorKind::UnterminatedString =>
                    "encountered unterminated string during tokenization".into(),
                ErrorKind::BadToken => "encountered bad token during tokenization".into(),
                ErrorKind::UnexpectedEnd => "encountered unexpected end-of-file".into(),
            }
        )
    }
}

impl std::error::Error for Error {}

/// type alias for a result using our error type
pub type Result<T> = std::result::Result<T, Error>;

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
