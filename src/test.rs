use crate::{args, ast, token::*, utils};
use utils::Location;

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

#[test]
fn token_ize() {
    assert_eq!(
        ize(
            "test.qq",
            "\"test\" 10 { , }\nquestion answer value title pass by"
        ),
        TokenStream::from([
            Token {
                kind: TokenKind::String("test".to_string()),
                begin: Location { line: 1, col: 1 },
                end: Location { line: 1, col: 6 },
            },
            Token {
                kind: TokenKind::Number(10,),
                begin: Location { line: 1, col: 7 },
                end: Location { line: 1, col: 9 },
            },
            Token {
                kind: TokenKind::LBrace,
                begin: Location { line: 1, col: 10 },
                end: Location { line: 1, col: 11 },
            },
            Token {
                kind: TokenKind::Comma,
                begin: Location { line: 1, col: 12 },
                end: Location { line: 1, col: 13 },
            },
            Token {
                kind: TokenKind::RBrace,
                begin: Location { line: 1, col: 14 },
                end: Location { line: 1, col: 15 },
            },
            Token {
                kind: TokenKind::Question,
                begin: Location { line: 2, col: 1 },
                end: Location { line: 2, col: 9 },
            },
            Token {
                kind: TokenKind::Answer,
                begin: Location { line: 2, col: 10 },
                end: Location { line: 2, col: 16 },
            },
            Token {
                kind: TokenKind::Value,
                begin: Location { line: 2, col: 17 },
                end: Location { line: 2, col: 22 },
            },
            Token {
                kind: TokenKind::Title,
                begin: Location { line: 2, col: 23 },
                end: Location { line: 2, col: 28 },
            },
            Token {
                kind: TokenKind::Pass,
                begin: Location { line: 2, col: 29 },
                end: Location { line: 2, col: 33 },
            },
            Token {
                kind: TokenKind::By,
                begin: Location { line: 2, col: 34 },
                end: Location { line: 2, col: 36 },
            },
        ])
    );
}

#[test]
fn ast_ify() {
    assert_eq!(
        ast::ify(
            &mut ize(
                "test.qq",
                r#"title "test quiz" by "sarkar-segfault" pass 3

                question "does life have any meaning?" {
	                answer {
		                "no"
	                }
	                value 3
                }"#
            ),
            "test.qq"
        ),
        ast::Quiz {
            questions: vec![ast::Question {
                answer: vec!["no".to_string()],
                text: "does life have any meaning?".to_string(),
                value: 3
            }],
            metaline: ast::Metaline {
                title: "test quiz".to_string(),
                by: "sarkar-segfault".to_string(),
                pass: 3
            }
        }
    );
}
