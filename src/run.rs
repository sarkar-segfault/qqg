use crate::utils::{Color, color};
use crate::{ast::Quiz, fatal};
use std::io::{Write, stdin, stdout};

pub fn start(quiz: Quiz) {
    println!(
        "{}\n{} {}\n{} {}\n",
        color(Color::Yellow, &quiz.metaline.title),
        color(Color::Grey, "by"),
        color(Color::Yellow, &quiz.metaline.by),
        color(Color::Grey, "passing marks"),
        color(Color::Yellow, &quiz.metaline.pass.to_string())
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
        print!("{} ", color(Color::SuperCyan, "└─"));
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
            println!("{}\n", color(Color::Green, "correct answer!"));
            score += question.value;
        } else {
            println!(
                "{} \n",
                color(
                    Color::Red,
                    &format!(
                        "wrong answer!\n└─ expected {}",
                        question
                            .answer
                            .iter()
                            .map(|s| format!("\"{}\"", s))
                            .collect::<Vec<_>>()
                            .join(" or ")
                    )
                )
            );
        }
    }

    let pass = score >= quiz.metaline.pass;

    println!(
        "{} {} {} {}",
        color(Color::Yellow, "you scored"),
        color(
            if pass { Color::Green } else { Color::Red },
            &score.to_string()
        ),
        color(Color::Yellow, "out of"),
        color(Color::SuperCyan, &total.to_string())
    );

    println!(
        "{}",
        if pass {
            color(Color::Green, "you passed!")
        } else {
            color(Color::Red, "you failed!")
        }
    );
}
