use crate::ast::Program;
use crate::err;
use std::io::{Write, stdin, stdout};

pub fn console(prog: Program) {
    println!("{} by {}\n", prog.meta.title, prog.meta.by);
    let mut score: isize = 0;
    let mut total: isize = 0;

    for question in prog.questions {
        print!("{}\n=> ", question.text);
        stdout().flush().unwrap_or_else(|e| err!(e));
        let mut answer = String::new();
        stdin().read_line(&mut answer).unwrap_or_else(|e| err!(e));
        let answer = answer.trim().to_string();

        if question.answer.contains(&answer) {
            print!("correct answer! ");
            score += question.value;
        } else {
            println!("\nwrong answer!\n=> {:?}", question.answer);
        }

        println!("score changed by {}\n", question.value);
        total += question.value;
    }

    println!("you scored {} out of {}", score, total);
}
