use crate::token::{Token, TokenKind};
use crate::utils::{Error, ErrorKind, Location, Result};

#[derive(Default)]
pub struct Meta {
    pub title: String,
    pub by: String,
}

#[derive(Default)]
pub struct Answer {
    pub has: Vec<String>,
    pub is: Vec<String>,
    pub show: bool,
}

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

#[derive(Default)]
pub struct Style {
    pub show: bool,
    pub styles: Vec<StyleKind>,
}

#[derive(Default)]
pub struct Question {
    pub text: String,
    pub answer: Answer,
    pub style: Style,
}

#[derive(Default)]
pub struct Program {
    pub meta: Meta,
    pub questions: Vec<Question>,
}

fn next(tokens: &mut Vec<Token>, loc: Location, file: &str) -> Result<Token> {
    match tokens.pop() {
        Some(token) => Ok(token),
        None => {
            return Err(Error {
                loc,
                file: file.into(),
                kind: ErrorKind::UnexpectedEnd,
            });
        }
    }
}

fn expect(
    kind: TokenKind,
    ekind: ErrorKind,
    tokens: &mut Vec<Token>,
    loc: Location,
    file: &str,
) -> Result<Token> {
    let token = next(tokens, loc, file)?;
    if token.kind != kind {
        Err(Error {
            loc,
            file: file.into(),
            kind: ekind,
        })
    } else {
        Ok(token)
    }
}

fn parse_meta(tokens: &mut Vec<Token>, loc: Location, file: &str) -> Result<Meta> {
    let mut meta = Meta::default();
    let title_string = next(tokens, loc, file)?;

    match title_string.kind {
        TokenKind::String(title) => meta.title = title,
        _ => {
            return Err(Error {
                loc,
                file: file.into(),
                kind: ErrorKind::ExpectedString,
            });
        }
    }

    let by_dir = expect(
        TokenKind::By,
        ErrorKind::UnexpectedToken,
        tokens,
        title_string.loc,
        file,
    )?;

    let by_string = next(tokens, by_dir.loc, file)?;

    match by_string.kind {
        TokenKind::String(by) => meta.by = by,
        _ => {
            return Err(Error {
                loc: by_string.loc,
                file: file.into(),
                kind: ErrorKind::ExpectedString,
            });
        }
    }

    Ok(meta)
}

fn parse_question(tokens: &mut Vec<Token>, loc: Location, file: &str) -> Result<Question> {
    let mut ques = Question::default();

    Ok(ques)
}

pub fn ify(tokens: &mut Vec<Token>, file: &str) -> Result<Program> {
    tokens.reverse();
    let mut prog = Program::default();

    while let Some(token) = tokens.pop() {
        match token.kind {
            TokenKind::Title => prog.meta = parse_meta(tokens, token.loc, file)?,
            TokenKind::Question => prog
                .questions
                .push(parse_question(tokens, token.loc, file)?),
            _ => {
                return Err(Error {
                    loc: token.loc,
                    file: file.into(),
                    kind: ErrorKind::UnexpectedToken,
                });
            }
        }
    }

    Ok(prog)
}
