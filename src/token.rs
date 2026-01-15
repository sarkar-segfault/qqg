use crate::{token_error, utils::Location};

#[derive(Debug, PartialEq, Eq)]
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
    Pass,
    By,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub begin: Location,
    pub end: Location,
}

pub type TokenStream = std::collections::VecDeque<Token>;

pub fn ize(file: &str, text: &str) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut chars = text.chars().peekable();
    let mut begin: Location;
    let mut loc = Location::default();
    let mut buf = String::new();

    while let Some(tok) = chars.next() {
        begin = loc;
        tokens.push_back(match tok {
            '{' => {
                loc.col += 1;
                Token {
                    kind: TokenKind::LBrace,
                    begin,
                    end: loc,
                }
            }
            '}' => {
                loc.col += 1;
                Token {
                    kind: TokenKind::RBrace,
                    begin,
                    end: loc,
                }
            }
            ',' => {
                loc.col += 1;
                Token {
                    kind: TokenKind::Comma,
                    begin,
                    end: loc,
                }
            }
            '"' => {
                let mut closed = false;

                for chr in chars.by_ref() {
                    loc.col += 1;
                    if chr == '"' {
                        closed = true;
                        break;
                    }
                    buf.push(chr);
                }

                if !closed {
                    token_error!(
                        begin,
                        loc,
                        &format!("encountered unterminated string: \"{}\n", buf),
                        file
                    );
                }

                let out = Token {
                    kind: TokenKind::String(buf.clone()),
                    begin,
                    end: loc,
                };

                buf.clear();
                out
            }
            '-' | '0'..='9' => {
                buf.push(tok);
                loc.col += 1;
                while let Some(&dig) = chars.peek()
                    && dig.is_numeric()
                {
                    chars.next();
                    loc.col += 1;
                    buf.push(dig);
                }

                let out = Token {
                    kind: TokenKind::Number(buf.parse::<isize>().unwrap_or_else(|e| {
                        token_error!(
                            begin,
                            loc,
                            &format!("failed to parse number {}: {}", buf, e),
                            file
                        )
                    })),
                    begin,
                    end: loc,
                };

                buf.clear();
                out
            }
            _ if tok.is_alphanumeric() => {
                buf.push(tok);
                loc.col += 1;

                while let Some(&chr) = chars.peek()
                    && (chr.is_alphanumeric())
                {
                    loc.col += 1;
                    chars.next();
                    buf.push(chr);
                }

                let out = Token {
                    kind: match buf.as_str() {
                        "question" => TokenKind::Question,
                        "answer" => TokenKind::Answer,
                        "value" => TokenKind::Value,
                        "title" => TokenKind::Title,
                        "pass" => TokenKind::Pass,
                        "by" => TokenKind::By,
                        _ => token_error!(
                            begin,
                            loc,
                            &format!("encountered unrecognized keyword: {}", buf),
                            file
                        ),
                    },
                    begin,
                    end: loc,
                };

                buf.clear();
                out
            }
            _ if matches!(tok, '\n' | '\r') => {
                loc.line += 1;
                loc.col = 1;
                continue;
            }
            _ if tok.is_whitespace() => {
                loc.col += 1;
                continue;
            }
            '#' => {
                for chr in chars.by_ref() {
                    if chr == '\n' {
                        loc.line += 1;
                        loc.col = 1;
                        break;
                    }
                }
                continue;
            }
            _ => token_error!(
                begin,
                loc,
                &format!("encountered unrecognized token: {}", tok),
                file
            ),
        });
    }

    tokens
}
