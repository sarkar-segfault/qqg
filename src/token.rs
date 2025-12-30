use crate::utils::{Error, ErrorKind, Location, Result};

#[derive(PartialEq, Debug, Eq, Clone)]
pub enum TokenKind {
    String(String),
    Number(isize),
    LBrace,
    RBrace,
    Comma,

    Question,
    Answer,
    Value,
    Title,
    By,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Location,
}

pub fn ize(input: String, file: &String) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut loc = Location { line: 1, col: 1 };

    while let Some(&c) = chars.peek() {
        tokens.push(match c {
            '{' => {
                let tloc = loc;
                chars.next();
                loc.col += 1;
                Token {
                    kind: TokenKind::LBrace,
                    loc: tloc,
                }
            }
            '}' => {
                let tloc = loc;
                chars.next();
                loc.col += 1;
                Token {
                    kind: TokenKind::RBrace,
                    loc: tloc,
                }
            }
            ',' => {
                let tloc = loc;
                chars.next();
                loc.col += 1;
                Token {
                    loc: tloc,
                    kind: TokenKind::Comma,
                }
            }
            '"' => {
                let tloc = loc;
                chars.next();
                loc.col += 1;

                let mut closed = false;
                let mut content = String::new();

                while let Some(&n) = chars.peek() {
                    if n == '"' {
                        closed = true;
                        chars.next();
                        loc.col += 1;
                        break;
                    } else {
                        content.push(n);
                        chars.next();
                        loc.col += 1;
                    }
                }

                if !closed {
                    return Err(Error {
                        loc,
                        file: file.into(),
                        kind: ErrorKind::UnterminatedString,
                    });
                }

                Token {
                    kind: TokenKind::String(content),
                    loc: tloc,
                }
            }
            '-' => {
                let tloc = loc;
                let mut content = String::new();
                chars.next();
                loc.col += 1;

                while let Some(&n) = chars.peek() {
                    if !n.is_numeric() {
                        break;
                    }
                    chars.next();
                    loc.col += 1;
                    content.push(n);
                }

                let number = content.parse::<isize>().map_err(|_| Error {
                    loc: tloc,
                    file: file.into(),
                    kind: ErrorKind::MalformedNumber,
                })?;

                Token {
                    kind: TokenKind::Number(-number),
                    loc: tloc,
                }
            }
            _ if c.is_numeric() => {
                let tloc = loc;
                let mut content = String::new();

                while let Some(&n) = chars.peek() {
                    if !n.is_numeric() {
                        break;
                    }
                    chars.next();
                    loc.col += 1;
                    content.push(n);
                }

                let number = content.parse::<isize>().map_err(|_| Error {
                    loc: tloc,
                    file: file.into(),
                    kind: ErrorKind::MalformedNumber,
                })?;
                Token {
                    kind: TokenKind::Number(number),
                    loc: tloc,
                }
            }
            _ if c.is_alphanumeric() => {
                let tloc = loc;
                let mut content = String::new();

                while let Some(&n) = chars.peek() {
                    if !n.is_alphanumeric() {
                        break;
                    }
                    content.push(n);
                    chars.next();
                    loc.col += 1;
                }

                let kind = match content.as_str() {
                    "question" => TokenKind::Question,
                    "answer" => TokenKind::Answer,
                    "value" => TokenKind::Value,
                    "title" => TokenKind::Title,
                    "by" => TokenKind::By,
                    _ => {
                        return Err(Error {
                            loc,
                            file: file.into(),
                            kind: ErrorKind::InvalidKeyword(content),
                        });
                    }
                };
                Token { kind, loc: tloc }
            }
            _ if c == '#' => {
                while let Some(&n) = chars.peek() {
                    chars.next();
                    loc.col += 1;
                    if n == '\n' {
                        break;
                    }
                }
                continue;
            }
            _ if c == '\n' => {
                chars.next();
                loc.line += 1;
                loc.col = 1;
                continue;
            }
            _ if c.is_whitespace() => {
                chars.next();
                loc.col += 1;
                continue;
            }
            _ => {
                return Err(Error {
                    loc,
                    file: file.into(),
                    kind: ErrorKind::UnrecognizedToken(c.into()),
                });
            }
        });
    }

    Ok(tokens)
}
