# quick-quiz-generator
qqg is a rust program designed to let you create interactive console-based quizzes blazingly fast.

# features
- simple and portable text format for structured quizzes
- free and open-source, under GPL-3.0-or-later
- fast, handmade parser with robust lexing
- no-dependencies, only `std`

# install
using cargo, you can run:
```bash
cargo install quick-quiz-generator
```

# usage
qqg parses `.qq` files, and runs them as quizzes. to get started, create a file `test.qq` and write:
```quick-quiz
title "quick-quiz-generator" by "sarkar-segfault"

question "what is 1 + 1?" {
	answer {
  	"2",  
  },
	value 1,
}
```

run it using the following command:
```bash
qqg -i test.qq
```

for some sample quizzes, see `samples/`.

# commands
run the following command for help about the cli:
```bash
qqg -h
```
