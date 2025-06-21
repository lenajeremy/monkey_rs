#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(tt: TokenType, literal: String) -> Self {
        Token {
            token_type: tt,
            literal: literal,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    Identifier,
    Integer,

    Assign,
    Plus,

    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EOF => "EOF",
            Self::Function => "FUNCTION",
            Self::LBrace => "LBRACE",
            Self::RBrace => "RBRACE",
            Self::LParen => "LPAREN",
            Self::RParen => "RPAREN",
            Self::Plus => "Plus",
            Self::Assign => "Assign",
            Self::Illegal => "ILLEGAL",
            Self::Integer => "INTEGER",
            Self::Identifier => "IDENTIFIER",
            Self::Semicolon => "SEMICOLON",
            Self::Comma => "COMMA",
            Self::Let => "LET",
        }
    }
}
