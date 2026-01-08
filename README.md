# quick-quiz-generator
qqg is a rust program designed to let you create interactive quizzes blazingly fast, right in the terminal.

## version 1.0.0
qqg version 1.0.0 is the second breaking release and rewrite of qqg, after 0.1.0, filled with new features and code improvements.

# features
- simple and portable text format for structured quizzes
- free and open-source, under GPL-3.0-or-later
- fast, handmade parser with robust lexing
- colored output and symbols
- 0 dependencies, only `std`

> [!TIP]
> if your stdout is not a terminal, or the `NO_COLOR` environment variable is set, qqg doesn't use colors

# install
using cargo, you can run:
```bash
cargo install quick-quiz-generator
```
and boom, you can invoke it as `qqg`

# usage
qqg parses `.qq` (quick-quiz) files, and runs them as quizzes. to get started, create a file `test.qq` and write:
```quick-quiz
title "quick-quiz-generator" by "sarkar-segfault"

question "what is 1 + 1?" {
	answer {
  	"2"
  }
	value 1
}
```

> [!TIP]
> qqg supports commas, unicode strings, and negative values in input too.

run it using the following command:
```bash
qqg start test.qq
```

for some sample quizzes, see `samples/`.

# commands
qqg has 4 main subcommands:
| subcommand | description |
|---------|-------------|
| `help`  | prints help about the cli |
| `token <input.qq>` | tokenizes the file and prints its token list |
| `parse <input.qq>` | tokenizes and parses the file and prints the syntax tree |
| `start <input.qq>` | tokenizes, parses and runs the file as an interactive quiz |
most of the time, you'll only use `help` and `start`; the others are mostly for testing.

# contribution
qqg was started as a one-man project by me, but i would appreciate any help i can get! just make an issue or pull request at the [repo](https://github.com/sarkar-segfault/qqg), and i promise i'll check it out.
