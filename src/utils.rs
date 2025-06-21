use std::collections::HashMap;

use crate::token::TokenType;

pub fn is_letter(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => ch.is_alphabetic(),
        None => false,
    }
}

pub fn is_white_space(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => ch.is_whitespace(),
        None => false,
    }
}

pub fn is_number(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => ch.is_numeric(),
        None => false,
    }
}

pub fn keywords() -> HashMap<String, TokenType> {
    let hashmap = vec![
        (String::from("fn"), TokenType::Function),
        (String::from("let"), TokenType::Let),
    ]
    .into_iter()
    .collect();
    hashmap
}
