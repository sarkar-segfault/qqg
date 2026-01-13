use crate::{args, ast, token::*, utils};

#[test]
fn args_parse() {
    assert_eq!(
        args::parse(
            vec![
                String::from("qqg"),
                String::from("start"),
                String::from("input.qq")
            ]
            .into_iter()
        ),
        args::Info {
            cmd: args::Command::Start,
            file: "input.qq".to_string()
        }
    );
}

#[test]
fn utils_color() {
    if utils::color(utils::Color::Red, "test") != "test" {
        unsafe {
            std::env::set_var("NO_COLOR", "1");
        }

        assert_eq!(utils::color(utils::Color::Red, "test"), "test");
    } else {
        unsafe {
            std::env::remove_var("NO_COLOR");
        }

        assert_eq!(
            utils::color(utils::Color::Red, "test"),
            "\x1b[31mtest\x1b[0m"
        );
    }
}

fn mkloc(line: usize, col: usize) -> utils::Location {
    utils::Location { line, col }
}

fn mktok(kind: TokenKind, begin: utils::Location, end: utils::Location) -> Token {
    Token { kind, begin, end }
}

#[test]
fn token_ize() {
    assert_eq!(
        ize(
            "test.qq",
            "\"test\" 10 {{ , }} question answer value title by"
        ),
        TokenStream::from([
            mktok(
                TokenKind::String("test".to_string()),
                mkloc(1, 1),
                mkloc(1, 6)
            ),
            mktok(TokenKind::Number(10), mkloc(1, 7), mkloc(1, 9)),
            mktok(TokenKind::LBrace, mkloc(1, 10), mkloc(1, 10)),
            mktok(TokenKind::LBrace, mkloc(1, 11), mkloc(1, 11)),
            mktok(TokenKind::Comma, mkloc(1, 13), mkloc(1, 13)),
            mktok(TokenKind::RBrace, mkloc(1, 15), mkloc(1, 15)),
            mktok(TokenKind::RBrace, mkloc(1, 16), mkloc(1, 16)),
            mktok(TokenKind::Question, mkloc(1, 18), mkloc(1, 26)),
            mktok(TokenKind::Answer, mkloc(1, 27), mkloc(1, 33)),
            mktok(TokenKind::Value, mkloc(1, 34), mkloc(1, 39)),
            mktok(TokenKind::Title, mkloc(1, 40), mkloc(1, 45)),
            mktok(TokenKind::By, mkloc(1, 46), mkloc(1, 48)),
        ])
    );
}

#[test]
fn parse_ify() {
    assert_eq!(
        ast::ify(
            &mut ize(
                "test.qq",
                r#"title "test quiz" by "sarkar-segfault"

                question "does life have any meaning?" {{
	                answer {{
		                "no"
	                }}
	                value 3
                }}
                "#
            ),
            "test.qq"
        ),
        ast::Quiz {
            questions: vec![ast::Question {
                answer: vec!["no".to_string()],
                text: "does life have any meaning".to_string(),
                value: 3
            }],
            metaline: ast::Metaline {
                title: "test quiz".to_string(),
                by: "sarkar-segfault".to_string()
            }
        }
    );
}
