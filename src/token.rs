use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Token {
    String(String),
    LBrace,
    RBrace,
    Comma,

    Question,
    Answer,
    Style,
    Title,
    Show,
    Has,
    In,
    By,

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

#[derive(Debug)]
pub enum ErrorKind {
    Token,
    String,
    Keyword,
}

#[derive(Debug)]
pub struct Error {
    file: String,
    line: usize,
    col: usize,
    err: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self.err {
            ErrorKind::Token => "invalid token",
            ErrorKind::String => "invalid string",
            ErrorKind::Keyword => "invalid keyword",
        };

        write!(f, "[{}:{}:{}] {}", self.file, self.line, self.col, msg)
    }
}

impl std::error::Error for Error {}

pub type Result = std::result::Result<Vec<Token>, Error>;

pub fn ize(input: String, file: String) -> Result {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line: usize = 1;
    let mut col: usize = 1;

    while let Some(&c) = chars.peek() {
        tokens.push(match c {
            '{' => {
                chars.next();
                col += 1;
                Token::LBrace
            }
            '}' => {
                chars.next();
                col += 1;
                Token::RBrace
            }
            ',' => {
                chars.next();
                col += 1;
                Token::Comma
            }
            '"' => {
                chars.next();
                col += 1;

                let mut closed = false;
                let mut content = String::new();

                while let Some(&n) = chars.peek() {
                    if n == '"' {
                        closed = true;
                        chars.next();
                        col += 1;
                        break;
                    } else {
                        content.push(n);
                        chars.next();
                        col += 1;
                    }
                }

                if !closed {
                    return Err(Error {
                        file,
                        line,
                        col,
                        err: ErrorKind::String,
                    });
                }

                Token::String(content)
            }
            _ if c.is_alphanumeric() => {
                let mut content = String::new();

                while let Some(&n) = chars.peek() {
                    if !n.is_alphanumeric() {
                        break;
                    }
                    content.push(n);
                    chars.next();
                    col += 1;
                }

                match content.as_str() {
                    "question" => Token::Question,
                    "answer" => Token::Answer,
                    "style" => Token::Style,
                    "title" => Token::Title,
                    "show" => Token::Show,
                    "has" => Token::Has,
                    "in" => Token::In,
                    "by" => Token::By,

                    "fg" => Token::Fg,
                    "bg" => Token::Bg,
                    "br" => Token::Br,

                    "magenta" => Token::Magenta,
                    "yellow" => Token::Yellow,
                    "green" => Token::Green,
                    "white" => Token::White,
                    "black" => Token::Black,
                    "blue" => Token::Blue,
                    "cyan" => Token::Cyan,
                    "red" => Token::Red,

                    "underline" => Token::Underline,
                    "strike" => Token::Strike,
                    "italic" => Token::Italic,
                    "invert" => Token::Invert,
                    "hidden" => Token::Hidden,
                    "blink" => Token::Blink,
                    "bold" => Token::Bold,
                    "dim" => Token::Dim,
                    _ => {
                        return Err(Error {
                            file,
                            line,
                            col,
                            err: ErrorKind::Keyword,
                        });
                    }
                }
            }
            _ if c == '\n' => {
                chars.next();
                line += 1;
                col = 1;
                continue;
            }
            _ if c.is_whitespace() => {
                chars.next();
                col += 1;
                continue;
            }
            _ => {
                return Err(Error {
                    file,
                    line,
                    col,
                    err: ErrorKind::Token,
                });
            }
        });
    }

    Ok(tokens)
}
