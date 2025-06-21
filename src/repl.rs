use std::io;

use crate::{lexer, token::TokenType};

//const PROMPT: &'static str = ">>";

pub fn scan() {
    loop {
        let mut s = String::new();
        let _ = io::stdin().read_line(&mut s);

        let mut l = lexer::Lexer::new(s);

        loop {
            let token = l.next_token();
            match token.token_type {
                TokenType::EOF => {
                    break;
                }
                _ => {
                    println!("{}, {}", token.literal, token.token_type.as_str());
                }
            }
        }
    }
}
