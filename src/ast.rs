//! defines abstract syntax tree components and parsing functions

use crate::token::{Token, TokenKind};
use crate::utils::{Error, ErrorKind, Location, Result};

/// contains information about the metaline (title + by)
#[derive(Default)]
pub struct Meta {
    pub title: String,
    pub by: String,
}

/// represents an answer directive
pub struct Answer {
    pub has: Vec<String>,
    pub is: Vec<String>,
    pub show: bool,
}

/// represents different styling features
pub enum StyleKind {
    Fg,
    Bg,
    Br,

    Magenta,
    Yellow,
    Green,
    White,
    Black,
    Blue,
    Cyan,
    Red,

    Underline,
    Strike,
    Italic,
    Invert,
    Hidden,
    Blink,
    Bold,
    Dim,
}

/// represents a style block
pub struct Style(bool, Vec<StyleKind>);

/// represents a question
pub struct Question {
    pub text: String,
    pub answer: Answer,
    pub style: Style,
}

/// contains information about the program
#[derive(Default)]
pub struct Program {
    pub meta: Meta,
    pub questions: Vec<Question>,
}

/// helper for getting next token outside of loop
fn next(tokens: &mut Vec<Token>, last_loc: Location, file: &String) -> Result<Token> {
    match tokens.pop() {
        Some(t) => Ok(t),
        None => Err(Error {
            loc: last_loc,
            file: file.into(),
            kind: ErrorKind::UnexpectedEnd,
        }),
    }
}

/// parse a metaline (title + by)
fn parse_meta(tokens: &mut Vec<Token>, last_loc: Location, file: &String) -> Result<Meta> {
    let token1 = next(tokens, last_loc, file)?;
    let mut meta = Meta::default();

    match token1.kind {
        TokenKind::String(s) => {
            meta.title = s;
        }
        _ => {
            return Err(Error {
                loc: token1.loc,
                file: file.into(),
                kind: ErrorKind::UnexpectedToken(
                    "String(...)".into(),
                    format!("{:?}", token1.kind),
                ),
            });
        }
    }

    let token2 = match next(tokens, token1.loc, file) {
        Ok(t) => t,
        Err(_) => return Ok(meta),
    };

    if token2.kind == TokenKind::By {
        let token3 = next(tokens, token2.loc, file)?;
        match token3.kind {
            TokenKind::String(s) => meta.by = s,
            _ => {
                return Err(Error {
                    loc: token3.loc,
                    file: file.into(),
                    kind: ErrorKind::UnexpectedToken(
                        "String(...)".into(),
                        format!("{:?}", token3.kind),
                    ),
                });
            }
        }
    }

    Ok(meta)
}

/// convert a [`Vec`] of [`Token`] into a [`Result`] of [`Program`]
pub fn ify(mut tokens: Vec<Token>, file: &String) -> Result<Program> {
    let mut prog = Program::default();
    tokens.reverse();

    while let Some(token) = tokens.pop() {
        match token.kind {
            TokenKind::Title => prog.meta = parse_meta(&mut tokens, token.loc, file)?,
            _ => {
                return Err(Error {
                    loc: token.loc,
                    file: file.into(),
                    kind: ErrorKind::UnexpectedToken(
                        "Question or Title".into(),
                        format!("{:?}", token.kind),
                    ),
                });
            }
        }
    }

    Ok(prog)
}
