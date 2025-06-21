use std::fs;

mod lexer;
mod token;
mod utils;

fn main() {
    let src = fs::read_to_string("index.monkey").unwrap();
    let mut lexer = lexer::Lexer::new(src);

    lexer.read_char();
}
