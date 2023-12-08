use std::convert::Into;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT, // add, foobar, x, y
    INT,   // 123

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    //
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

    OTHER,
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value {
            "=" => TokenType::ASSIGN,
            "+" => TokenType::PLUS,
            "-" => TokenType::MINUS,
            "!" => TokenType::BANG,
            "*" => TokenType::ASTERISK,
            "/" => TokenType::SLASH,
            "<" => TokenType::LT,
            ">" => TokenType::GT,
            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "==" => TokenType::EQ,
            "!=" => TokenType::NEQ,
            _ => unreachable!(),
        }
    }
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

#[derive(PartialEq, Debug)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: String,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        let literal: String = value.into();
        Self {
            token_type: literal.as_str().into(),
            literal,
        }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Self {
            token_type: value.into(),
            literal: value.into(),
        }
    }
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
