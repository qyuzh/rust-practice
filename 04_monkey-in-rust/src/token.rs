use std::convert::Into;

use crate::impl_display_for_struct;

#[derive(PartialEq, Debug, Clone, Eq, Hash, Copy)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident, // add, foobar, x, y
    Int,   // 123

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,

    Assign,
    Bang,

    // Comparators
    LT,
    GT,

    Eq,
    NEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl TokenType {
    pub fn lookup_keyword(s: &str) -> Option<TokenType> {
        match s {
            "fn" => Some(TokenType::Function),
            "let" => Some(TokenType::Let),
            "true" => Some(TokenType::True),
            "false" => Some(TokenType::False),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "return" => Some(TokenType::Return),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

impl_display_for_struct!(Token: literal:);

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn eof() -> Token {
        Token::new(TokenType::Eof, "".into())
    }

    pub fn illegal(ch: char) -> Token {
        Token::new(TokenType::Illegal, ch.into())
    }

    pub fn number(s: String) -> Token {
        Token::new(TokenType::Int, s)
    }

    pub fn ident(s: String) -> Token {
        Token::new(TokenType::Ident, s)
    }
}
