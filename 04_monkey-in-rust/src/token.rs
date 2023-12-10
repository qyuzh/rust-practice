use std::convert::Into;

#[derive(PartialEq, Debug, Clone, Eq, Hash, Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT, // add, foobar, x, y
    INT,   // 123

    // Operators
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,

    ASSIGN,
    BANG,

    // Comparators
    LT,
    GT,

    EQ,
    NEQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl TokenType {
    pub fn lookup_keyword(s: &str) -> Option<TokenType> {
        match s {
            "fn" => Some(TokenType::FUNCTION),
            "let" => Some(TokenType::LET),
            "true" => Some(TokenType::TRUE),
            "false" => Some(TokenType::FALSE),
            "if" => Some(TokenType::IF),
            "else" => Some(TokenType::ELSE),
            "return" => Some(TokenType::RETURN),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn eof() -> Token {
        Token::new(TokenType::EOF, "".into())
    }

    pub fn illegal(ch: char) -> Token {
        Token::new(TokenType::ILLEGAL, ch.into())
    }

    pub fn number(s: String) -> Token {
        Token::new(TokenType::INT, s)
    }

    pub fn ident(s: String) -> Token {
        Token::new(TokenType::IDENT, s)
    }
}
