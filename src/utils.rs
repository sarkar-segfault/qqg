use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum ErrorKind {
    InvalidKeyword(String),
    UnterminatedString,
    UnrecognizedToken(String),
    UnexpectedToken,
    UnexpectedEnd,
    ExpectedString,
    MissingBrace,
    MalformedNumber,
    ExpectedComma,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

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
                ErrorKind::InvalidKeyword(kw) => format!("encountered invalid keyword: {}", kw),
                ErrorKind::UnterminatedString =>
                    "encountered unterminated string during tokenization".into(),
                ErrorKind::UnrecognizedToken(ch) =>
                    format!("encountered unrecognized token during tokenization: {}", ch),
                ErrorKind::UnexpectedToken => "encountered unexpected token during parsing".into(),
                ErrorKind::UnexpectedEnd =>
                    "encountered unexpected end-of-file during parsing".into(),
                ErrorKind::MissingBrace => "expected brace during parsing".into(),
                ErrorKind::MalformedNumber =>
                    "encountered malformed number during toknization".into(),
                ErrorKind::ExpectedComma => "expected comma during parsing".into(),
                ErrorKind::ExpectedString => "expected string during parsing".into(),
            }
        )
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

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
