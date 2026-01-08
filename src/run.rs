use crate::utils::{Color, color};
use crate::{ast::Quiz, fatal};
use std::io::{Write, stdin, stdout};

pub fn start(quiz: Quiz) {
    println!(
        "{}\n{} {}\n",
        color(Color::Yellow, &quiz.metaline.title),
        color(Color::Grey, "by"),
        color(Color::Yellow, &quiz.metaline.by)
    );

    let mut total = 0;
    let mut score = 0;

    for question in quiz.questions {
        total += question.value;
        println!(
            "{} {}",
            color(Color::SuperCyan, &question.text),
            color(Color::Grey, &format!("[{}]", question.value))
        );
        print!("{}", color(Color::SuperCyan, "└─"));
        stdout().flush().unwrap_or_else(|e| {
            fatal!(
                "{}",
                color(Color::Red, &format!("failed to flush stdout: {}", e))
            )
        });

        let mut answer = String::new();
        stdin().read_line(&mut answer).unwrap_or_else(|e| {
            fatal!(
                "{}",
                color(Color::Red, &format!("failed to read stdin: {}", e))
            )
        });
        answer = answer.trim().to_string();

        if question.answer.contains(&answer) {
            println!("{}\n", color(Color::Green, "you answered correctly!"));
            score += question.value;
        } else {
            println!(
                "{}\n",
                color(
                    Color::Red,
                    &format!("you answered wrong\n└─{:?}", question.answer)
                )
            );
        }
    }

    println!(
        "{}",
        color(
            Color::Yellow,
            &format!("you scored {} out of {}", score, total)
        )
    );
}
