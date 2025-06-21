use crate::token::{Token, TokenType};
use crate::utils::{is_letter, is_white_space,is_number, keywords};

#[derive(Debug)]
pub struct Lexer {
    src: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    fn eat_whitespace(&mut self) {
        while is_white_space(self.ch) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let tok = match self.ch {
            Some('=') => Token {
                literal: "=".to_string(),
                token_type: TokenType::Assign,
            },
            Some('+') => Token {
                literal: "+".to_string(),
                token_type: TokenType::Plus,
            },
            Some(',') => Token {
                literal: ",".to_string(),
                token_type: TokenType::Comma,
            },
            Some(';') => Token {
                literal: ";".to_string(),
                token_type: TokenType::Semicolon,
            },
            Some('(') => Token {
                token_type: TokenType::LParen,
                literal: "(".to_string(),
            },
            Some(')') => Token {
                token_type: TokenType::RParen,
                literal: ")".to_string(),
            },
            Some('{') => Token {
                token_type: TokenType::LBrace,
                literal: "{".to_string(),
            },
            Some('}') => Token {
                token_type: TokenType::RBrace,
                literal: "}".to_string(),
            },
            None => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
            _ => {
                if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    let keywords = keywords();
                    let token_type = keywords.get(&identifier).unwrap_or(&TokenType::Identifier);

                    return Token {
                        token_type: token_type.clone(),
                        literal: identifier,
                    };
                } else if is_number(self.ch) {
                    let integer_literal = self.read_integer();
                    return Token {
                        token_type: TokenType::Integer,
                        literal: integer_literal,
                    };
                } else {
                    return Token {
                        token_type: TokenType::Illegal,
                        literal: "".to_string(),
                    };
                }
            }
        };
        
        self.read_char();
        tok
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.src.len() {
            self.ch = None;
        } else {
            self.ch = self.src[self.read_position..].chars().next();

            self.position = self.read_position;
            self.read_position += self.ch.unwrap().len_utf8();
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        let s = &self.src[position..self.position];
        String::from(s)
    }

    pub fn read_integer(&mut self) -> String {
        let position = self.position;

        while is_number(self.ch) {
            self.read_char();
        }

        let s = &self.src[position..self.position];
        String::from(s)
    }

    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            src: input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new(String::from("=+(){},;"));
        let expected_tokens = vec![
            Token {
                literal: "=".to_string(),
                token_type: TokenType::Assign,
            },
            Token {
                literal: "+".to_string(),
                token_type: TokenType::Plus,
            },
            Token {
                literal: "(".to_string(),
                token_type: TokenType::LParen,
            },
            Token {
                literal: ")".to_string(),
                token_type: TokenType::RParen,
            },
            Token {
                literal: "{".to_string(),
                token_type: TokenType::LBrace,
            },
            Token {
                literal: "}".to_string(),
                token_type: TokenType::RBrace,
            },
            Token {
                literal: ",".to_string(),
                token_type: TokenType::Comma,
            },
            Token {
                literal: ";".to_string(),
                token_type: TokenType::Semicolon,
            },
        ];

        for token in expected_tokens {
            let n_token = lexer.next_token();
            assert_eq!(n_token.literal, token.literal)
        }
    }

    #[test]
    fn test_next_token_2() {
        let input = "let five = fifteen;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        }

        let result = add(five, ten);
        ";

        let mut lexer = Lexer::new(input.to_string());

        let expected_tokens = vec![
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "five".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Identifier, "fifteen".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "ten".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Identifier, "y".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Identifier, "y".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Identifier, "add".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Identifier, "five".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Identifier, "ten".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
        ];

        for token in expected_tokens {
            println!("{:?}", lexer.ch);
            let next_token = lexer.next_token();

            println!("EXPECTED: {token:#?}\nGOT: {next_token:#?}");

            assert_eq!(token.token_type.as_str(), next_token.token_type.as_str());
            assert_eq!(token.literal, next_token.literal);
        }
    }
}
