use crate::parse_error;
use crate::token::{Token, TokenKind, TokenStream};

#[derive(Debug, Default)]
pub struct Answer {
    pub valid: Vec<String>,
    pub option: bool,
}

#[derive(Debug, Default)]
pub struct Question {
    pub answer: Answer,
    pub value: isize,
}

#[derive(Debug, Default)]
pub struct Metaline {
    pub title: String,
    pub by: String,
}

#[derive(Debug, Default)]
pub struct Quiz {
    pub questions: Vec<Question>,
    pub metaline: Metaline,
}

fn next(tokens: &mut TokenStream, file: &str, last: Token, want: &[TokenKind]) -> Token {
    match tokens.pop_front() {
        Some(token) => {
            if want.contains(&token.kind) {
                token
            } else {
                parse_error!(
                    token,
                    &format!(
                        "encountered unexpected token; expected {}{:?}",
                        if want.len() == 1 { "" } else { "one of " },
                        want
                    ),
                    file
                )
            }
        }
        None => {
            parse_error!(
                last,
                &format!(
                    "encountered unexpected end of input; expected {}{:?}",
                    if want.len() == 1 { "" } else { "one of " },
                    want
                ),
                file
            );
        }
    }
}

fn next_string(tokens: &mut TokenStream, file: &str, last: Token) -> Token {
    match tokens.pop_front() {
        Some(token) => match token.kind {
            TokenKind::String(_) => token,
            _ => parse_error!(token, "", file),
        },
        None => parse_error!(
            last,
            "encountered unexpected end of input; expected [String]",
            file
        ),
    }
}

fn ify_question(tokens: &mut TokenStream, last: Token, file: &str) -> Question {
    todo!();
}

fn ify_metaline(tokens: &mut TokenStream, last: Token, file: &str) -> Metaline {
    let title = next_string(tokens, file, last);
    let mut metaline = Metaline::default();
    if let TokenKind::String(ref s) = title.kind {
        metaline.title = s.to_string();
    } else {
        unreachable!();
    }

    let by = next(tokens, file, title, &[TokenKind::By]);
    let bystr = next_string(tokens, file, by);
    match bystr.kind {
        TokenKind::String(s) => metaline.by = s,
        _ => unreachable!(),
    }

    metaline
}

pub fn ify(tokens: &mut TokenStream, file: &str) -> Quiz {
    let mut quiz = Quiz::default();

    while let Some(token) = tokens.pop_front() {
        match token.kind {
            TokenKind::Title => quiz.metaline = ify_metaline(tokens, token, file),
            TokenKind::Question => quiz.questions.push(ify_question(tokens, token, file)),
            _ => parse_error!(
                token,
                "encountered unexpected top-level directive; expected metaline or question",
                file
            ),
        }
    }

    quiz
}
