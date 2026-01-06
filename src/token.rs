use crate::utils::*;

#[derive(Debug)]
pub enum TokenKind {
    String(String),
    Number(isize),
    LBrace,
    RBrace,
    Comma,

    Question,
    Answer,
    Style,
    Value,
    Title,
    Show,
    By,

    FgBlack,
    FgRed,
    FgGreen,
    FgYellow,
    FgBlue,
    FgMagenta,
    FgCyan,
    FgWhite,

    FgBrBlack,
    FgBrRed,
    FgBrGreen,
    FgBrYellow,
    FgBrBlue,
    FgBrMagenta,
    FgBrCyan,
    FgBrWhite,

    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,

    BgBrBlack,
    BgBrRed,
    BgBrGreen,
    BgBrYellow,
    BgBrBlue,
    BgBrMagenta,
    BgBrCyan,
    BgBrWhite,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub begin: Location,
    pub end: Location,
}

pub type TokenStream = std::collections::VecDeque<Token>;

fn ize_number(
    loc: &mut Location,
    buf: &mut String,
    begin: Location,
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    file: &str,
) -> Token {
    while let Some(&dig) = chars.peek()
        && dig.is_numeric()
    {
        chars.next();
        loc.col += 1;
        buf.push(dig);
    }

    let out = Token {
        kind: TokenKind::Number(buf.parse::<isize>().unwrap_or_else(|e| {
            error(
                begin,
                *loc,
                &format!("failed to parse number: {}", e),
                file,
                false,
            )
        })),
        begin,
        end: *loc,
    };

    buf.clear();
    out
}

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
                    end: begin,
                }
            }
            '}' => {
                loc.col += 1;
                Token {
                    kind: TokenKind::RBrace,
                    begin,
                    end: begin,
                }
            }
            ',' => {
                loc.col += 1;
                Token {
                    kind: TokenKind::Comma,
                    begin,
                    end: begin,
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
                    error(begin, loc, "encountered unterminated string", file, false);
                }

                let out = Token {
                    kind: TokenKind::String(buf.clone()),
                    begin,
                    end: loc,
                };

                buf.clear();
                out
            }
            '-' => {
                buf.push(tok);
                ize_number(&mut loc, &mut buf, begin, &mut chars, file)
            }
            '0'..='9' => {
                buf.push(tok);
                ize_number(&mut loc, &mut buf, begin, &mut chars, file)
            }
            _ if tok.is_alphanumeric() => {
                buf.push(tok);

                while let Some(&chr) = chars.peek()
                    && (chr.is_alphanumeric() || chr == '_')
                {
                    loc.col += 1;
                    chars.next();
                    buf.push(chr);
                }

                let out = Token {
                    kind: match buf.as_str() {
                        "question" => TokenKind::Question,
                        "answer" => TokenKind::Answer,
                        "style" => TokenKind::Style,
                        "value" => TokenKind::Value,
                        "title" => TokenKind::Title,
                        "show" => TokenKind::Show,
                        "by" => TokenKind::By,

                        "fg_black" => TokenKind::FgBlack,
                        "fg_red" => TokenKind::FgRed,
                        "fg_green" => TokenKind::FgGreen,
                        "fg_yellow" => TokenKind::FgYellow,
                        "fg_blue" => TokenKind::FgBlue,
                        "fg_magenta" => TokenKind::FgMagenta,
                        "fg_cyan" => TokenKind::FgCyan,
                        "fg_white" => TokenKind::FgWhite,

                        "fg_br_black" => TokenKind::FgBrBlack,
                        "fg_br_red" => TokenKind::FgBrRed,
                        "fg_br_green" => TokenKind::FgBrGreen,
                        "fg_br_yellow" => TokenKind::FgBrYellow,
                        "fg_br_blue" => TokenKind::FgBrBlue,
                        "fg_br_magenta" => TokenKind::FgBrMagenta,
                        "fg_br_cyan" => TokenKind::FgBrCyan,
                        "fg_br_white" => TokenKind::FgBrWhite,

                        "bg_black" => TokenKind::BgBlack,
                        "bg_red" => TokenKind::BgRed,
                        "bg_green" => TokenKind::BgGreen,
                        "bg_yellow" => TokenKind::BgYellow,
                        "bg_blue" => TokenKind::BgBlue,
                        "bg_magenta" => TokenKind::BgMagenta,
                        "bg_cyan" => TokenKind::BgCyan,
                        "bg_white" => TokenKind::BgWhite,

                        "bg_br_black" => TokenKind::BgBrBlack,
                        "bg_br_red" => TokenKind::BgBrRed,
                        "bg_br_green" => TokenKind::BgBrGreen,
                        "bg_br_yellow" => TokenKind::BgBrYellow,
                        "bg_br_blue" => TokenKind::BgBrBlue,
                        "bg_br_magenta" => TokenKind::BgBrMagenta,
                        "bg_br_cyan" => TokenKind::BgBrCyan,
                        "bg_br_white" => TokenKind::BgBrWhite,
                        _ => error(begin, loc, "encountered unrecognized keyword", file, false),
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
            _ => error(begin, loc, "encountered unrecognized token", file, false),
        });
    }

    tokens
}
