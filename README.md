# monkey_rs

A lexer and REPL for Monkey, the small programming language from Thorsten Ball's *Writing an Interpreter in Go*, written in Rust.

## What works

- **Lexer** (`src/lexer.rs`): turns Monkey source into tokens: integers, identifiers, the keywords `fn`, `let`, `if`, `else`, `return`, `true` and `false`, the operators `= + - ! * / < > == !=` and the delimiters `, ; ( ) { }`. Two-character operators are read by peeking at the next character, and anything unrecognised becomes an `ILLEGAL` token.
- **Tests**: five table-driven tests cover each stage of the token set, from single characters to whole programs.
- **REPL** (`src/repl.rs`): reads a line and prints every token with its type.
- **AST** (`src/ast.rs`): the first `Node`, `Statement` and `Expression` traits for the parser.

The parser and evaluator aren't written yet. For a finished Pratt parser and tree-walking interpreter in Rust, see [marc](https://github.com/lenajeremy/marc).

## Run it

```sh
cargo test
cargo run
```

Then type some Monkey:

```
let add = fn(x, y) { x + y; };
let, LET
add, IDENTIFIER
=, Assign
fn, FUNCTION
(, LPAREN
x, IDENTIFIER
...
```

`index.monkey` has a longer sample program.
