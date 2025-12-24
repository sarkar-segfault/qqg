use std::fmt;

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
pub enum Error {
    InvalidToken,
    InvalidString,
    InvalidKeyword,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::InvalidToken => "invalid token",
            Error::InvalidString => "invalid string",
            Error::InvalidKeyword => "invalid keyword",
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {}

pub type Result = std::result::Result<Vec<Token>, Error>;

pub fn ize(input: &str) -> Result {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        tokens.push(match c {
            '{' => {
                chars.next();
                Token::LBrace
            }
            '}' => {
                chars.next();
                Token::RBrace
            }
            ',' => {
                chars.next();
                Token::Comma
            }
            '"' => {
                chars.next();
                let mut closed = false;
                let mut content = String::new();

                while let Some(&n) = chars.peek() {
                    if n == '"' {
                        closed = true;
                        chars.next();
                        break;
                    } else {
                        content.push(n);
                        chars.next();
                    }
                }

                if !closed {
                    return Err(Error::InvalidString);
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
                }

                match content.as_str() {
                    "question" => Token::Question,
                    "answer" => Token::Answer,
                    "style" => Token::Style,
                    "title" => Token::Title,
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
                    _ => return Err(Error::InvalidKeyword),
                }
            }
            _ if c.is_whitespace() => {
                chars.next();
                continue;
            }
            _ => {
                println!("{}", c);
                return Err(Error::InvalidToken);
            }
        });
    }

    Ok(tokens)
}
