use crate::utils::{Error, ErrorKind, Location, Result};

/// represents the different types of tokens
#[derive(PartialEq, Debug, Eq)]
pub enum TokenKind {
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
    Is,
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

/// represents a token, with location information
pub struct Token {
    pub kind: TokenKind,
    pub loc: Location,
}

/// tokenizes a String into a vector of [`Token`]
pub fn ize(input: String, file: String) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut loc = Location::default();

    while let Some(&c) = chars.peek() {
        tokens.push(match c {
            '{' => {
                chars.next();
                loc.col += 1;
                Token {
                    kind: TokenKind::LBrace,
                    loc,
                }
            }
            '}' => {
                chars.next();
                loc.col += 1;
                Token {
                    kind: TokenKind::RBrace,
                    loc,
                }
            }
            ',' => {
                chars.next();
                loc.col += 1;
                Token {
                    kind: TokenKind::Comma,
                    loc,
                }
            }
            '"' => {
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
                        file,
                        kind: ErrorKind::MalformedString,
                    });
                }

                Token {
                    kind: TokenKind::String(content),
                    loc,
                }
            }
            _ if c.is_alphanumeric() => {
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
                    "style" => TokenKind::Style,
                    "title" => TokenKind::Title,
                    "show" => TokenKind::Show,
                    "has" => TokenKind::Has,
                    "is" => TokenKind::Is,
                    "by" => TokenKind::By,

                    "fg" => TokenKind::Fg,
                    "bg" => TokenKind::Bg,
                    "br" => TokenKind::Br,

                    "magenta" => TokenKind::Magenta,
                    "yellow" => TokenKind::Yellow,
                    "green" => TokenKind::Green,
                    "white" => TokenKind::White,
                    "black" => TokenKind::Black,
                    "blue" => TokenKind::Blue,
                    "cyan" => TokenKind::Cyan,
                    "red" => TokenKind::Red,

                    "underline" => TokenKind::Underline,
                    "strike" => TokenKind::Strike,
                    "italic" => TokenKind::Italic,
                    "invert" => TokenKind::Invert,
                    "hidden" => TokenKind::Hidden,
                    "blink" => TokenKind::Blink,
                    "bold" => TokenKind::Bold,
                    "dim" => TokenKind::Dim,
                    _ => {
                        return Err(Error {
                            loc,
                            file,
                            kind: ErrorKind::InvalidKeyword,
                        });
                    }
                };
                Token { kind, loc }
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
                    file,
                    kind: ErrorKind::UnexpectedToken,
                });
            }
        });
    }

    Ok(tokens)
}
