use crate::token::{Token, TokenKind};
use crate::utils::{Error, ErrorKind, Location, Result};

#[derive(Debug, Default)]
pub struct Meta {
    pub title: String,
    pub by: String,
}

#[derive(Debug, Default)]
pub struct Answer {
    pub has: Vec<String>,
    pub is: Vec<String>,
}

pub type Style = Vec<TokenKind>;

#[derive(Debug, Default)]
pub struct Question {
    pub text: String,
    pub answer: Answer,
    pub style: Style,
}

#[derive(Debug, Default)]
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

fn parse_answer(tokens: &mut Vec<Token>, loc: Location, file: &str) -> Result<Answer> {
    let a_lbrace = expect(
        TokenKind::LBrace,
        ErrorKind::MissingBrace,
        tokens,
        loc,
        file,
    )?;
    let mut answ = Answer::default();
    let mut something = next(tokens, a_lbrace.loc, file)?;
    loop {
        match something.kind {
            TokenKind::RBrace => break,
            TokenKind::Has | TokenKind::Is => {
                let origin = something.clone();
                let a_lbrace2 = expect(
                    TokenKind::LBrace,
                    ErrorKind::MissingBrace,
                    tokens,
                    something.loc,
                    file,
                )?;
                something = next(tokens, a_lbrace2.loc, file)?;
                loop {
                    match something.kind {
                        TokenKind::String(ref s) => {
                            if origin.kind == TokenKind::Has {
                                answ.has.push(s.clone())
                            } else {
                                answ.is.push(s.clone())
                            }
                        }
                        TokenKind::RBrace => break,
                        _ => {
                            return Err(Error {
                                loc: something.loc,
                                file: file.into(),
                                kind: ErrorKind::UnexpectedToken,
                            });
                        }
                    }
                    something = next(tokens, something.loc, file)?;
                }
            }
            _ => {
                return Err(Error {
                    loc: something.loc,
                    file: file.into(),
                    kind: ErrorKind::UnexpectedToken,
                });
            }
        }
        something = next(tokens, something.loc, file)?;
    }

    Ok(answ)
}

fn parse_style(tokens: &mut Vec<Token>, loc: Location, file: &str) -> Result<Style> {
    let s_lbrace = expect(
        TokenKind::LBrace,
        ErrorKind::MissingBrace,
        tokens,
        loc,
        file,
    )?;
    let mut style = Style::default();
    let mut something = next(tokens, s_lbrace.loc, file)?;

    loop {
        match something.kind {
            TokenKind::RBrace => break,
            TokenKind::Bold
            | TokenKind::Italic
            | TokenKind::Underline
            | TokenKind::Strike
            | TokenKind::Invert
            | TokenKind::Hidden
            | TokenKind::Blink
            | TokenKind::Dim
            | TokenKind::Fg
            | TokenKind::Bg
            | TokenKind::Br
            | TokenKind::Magenta
            | TokenKind::Yellow
            | TokenKind::Green
            | TokenKind::White
            | TokenKind::Black
            | TokenKind::Blue
            | TokenKind::Cyan
            | TokenKind::Red => style.push(something.kind),
            _ => {
                return Err(Error {
                    loc: something.loc,
                    file: file.into(),
                    kind: ErrorKind::UnexpectedToken,
                });
            }
        }
        something = next(tokens, something.loc, file)?;
    }

    Ok(style)
}

fn parse_question(tokens: &mut Vec<Token>, loc: Location, file: &str) -> Result<Question> {
    let text = next(tokens, loc, file)?;
    let mut ques = Question::default();

    match text.kind {
        TokenKind::String(txt) => ques.text = txt,
        _ => {
            return Err(Error {
                loc: text.loc,
                file: file.into(),
                kind: ErrorKind::ExpectedString,
            });
        }
    }

    let q_lbrace = expect(
        TokenKind::LBrace,
        ErrorKind::MissingBrace,
        tokens,
        text.loc,
        file,
    )?;

    let mut something = next(tokens, q_lbrace.loc, file)?;
    loop {
        match something.kind {
            TokenKind::RBrace => break,
            TokenKind::Answer => ques.answer = parse_answer(tokens, something.loc, file)?,
            TokenKind::Style => ques.style = parse_style(tokens, something.loc, file)?,
            _ => {
                return Err(Error {
                    loc: something.loc,
                    file: file.into(),
                    kind: ErrorKind::UnexpectedToken,
                });
            }
        }

        something = next(tokens, something.loc, file)?;
    }

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
