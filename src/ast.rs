use crate::parse_error;
use crate::token::{Token, TokenKind, TokenStream};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Question {
    pub answer: Vec<String>,
    pub text: String,
    pub value: isize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Metaline {
    pub title: String,
    pub by: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Quiz {
    pub metaline: Metaline,
    pub questions: Vec<Question>,
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
            _ => parse_error!(
                token,
                "encountered unexpected token; expected [String]",
                file
            ),
        },
        None => parse_error!(
            last,
            "encountered unexpected end of input; expected [String]",
            file
        ),
    }
}

fn next_number(tokens: &mut TokenStream, last: Token, file: &str) -> Token {
    match tokens.pop_front() {
        Some(token) => match token.kind {
            TokenKind::Number(_) => token,
            _ => parse_error!(
                token,
                "encountered unexpected token; expected [Number]",
                file
            ),
        },
        None => parse_error!(
            last,
            "encountered unexpected end of input; expected [Number]",
            file
        ),
    }
}

fn ify_answer(tokens: &mut TokenStream, last: Token, file: &str) -> (Vec<String>, Token) {
    let mut stuff = next(tokens, file, last, &[TokenKind::LBrace]);
    let mut answer = Vec::<String>::new();

    while let Some(token) = tokens.pop_front() {
        match token.kind {
            TokenKind::RBrace => {
                stuff = token;
                break;
            }
            TokenKind::String(ref s) => {
                answer.push(s.to_string());
                stuff = token;

                if let Some(tok) = tokens.front()
                    && tok.kind == TokenKind::Comma
                {
                    stuff = tokens.pop_front().unwrap_or_else(|| unreachable!());
                }
            }
            _ => parse_error!(
                token,
                "encountered unexpected item; expected one of [String, Show]",
                file
            ),
        }
    }

    if stuff.kind != TokenKind::RBrace {
        parse_error!(stuff, "encountered unterminated [Answer] directive", file);
    }

    if answer.is_empty() {
        parse_error!(stuff, "expected [String]s in [Answer] directive", file);
    }

    if let Some(token) = tokens.front()
        && token.kind == TokenKind::Comma
    {
        tokens.pop_front();
    }

    (answer, stuff)
}

fn ify_question(tokens: &mut TokenStream, last: Token, file: &str) -> Question {
    let mut stuff = next_string(tokens, file, last);
    let mut question = Question::default();
    let mut closed = false;

    if let TokenKind::String(ref s) = stuff.kind {
        question.text = s.to_string();
    } else {
        unreachable!();
    }

    stuff = next(tokens, file, stuff, &[TokenKind::LBrace]);

    while let Some(token) = tokens.pop_front() {
        match token.kind {
            TokenKind::RBrace => {
                closed = true;
                break;
            }
            TokenKind::Value => {
                stuff = next_number(tokens, token, file);

                if let TokenKind::Number(ref n) = stuff.kind {
                    question.value = *n;
                } else {
                    unreachable!();
                }

                if let Some(tok) = tokens.front()
                    && tok.kind == TokenKind::Comma
                {
                    stuff = tokens.pop_front().unwrap_or_else(|| unreachable!());
                }
            }
            TokenKind::Answer => (question.answer, stuff) = ify_answer(tokens, token, file),
            _ => parse_error!(
                token,
                "encountered unexpected directive; expected one of [Answer, Value]",
                file
            ),
        }
    }

    if !closed {
        parse_error!(stuff, "encountered unterminated [Question] directive", file);
    }

    question
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
                "encountered unexpected top-level directive; expected one of [Title, Question]",
                file
            ),
        }
    }

    quiz
}
