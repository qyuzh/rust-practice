//! `source code` -lexer-> `tokens`
//! This transformation, from source code to tokens, is called *lexical analysis*
//! or *lexing* for short. It's done by a lexer(also called tokenizer or scanner).

use std::str::Chars;

use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: Chars<'a>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut instance = Self {
            input: input.chars(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        instance.read_char();
        instance
    }

    pub fn next_token(&mut self) -> Token {
        let tok;
        self.skip_white_space();
        match self.ch {
            '+' => tok = Token::new(TokenType::Plus, self.ch.into()),
            '-' => tok = Token::new(TokenType::Minus, self.ch.into()),
            '*' => tok = Token::new(TokenType::Asterisk, self.ch.into()),
            '/' => tok = Token::new(TokenType::Slash, self.ch.into()),
            '<' => tok = Token::new(TokenType::LT, self.ch.into()),
            '>' => tok = Token::new(TokenType::GT, self.ch.into()),
            '(' => tok = Token::new(TokenType::LParen, self.ch.into()),
            ')' => tok = Token::new(TokenType::RParen, self.ch.into()),
            '{' => tok = Token::new(TokenType::LBrace, self.ch.into()),
            '}' => tok = Token::new(TokenType::RBrace, self.ch.into()),
            ',' => tok = Token::new(TokenType::Comma, self.ch.into()),
            ';' => tok = Token::new(TokenType::Semicolon, self.ch.into()),
            '=' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    tok = Token::new(TokenType::Eq, "==".into());
                }
                _ => tok = Token::new(TokenType::Assign, self.ch.into()),
            },
            '!' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    tok = Token::new(TokenType::NEq, "!=".into());
                }
                _ => tok = Token::new(TokenType::Bang, self.ch.into()),
            },
            a if is_letter(a) => {
                // return directly, otherwise, line-63 will eat next char.
                let ident = self.read_identifier();
                if let Some(keyword_type) = TokenType::lookup_keyword(ident.as_str()) {
                    // println!("{keyword_type:?} {ident}");
                    return Token::new(keyword_type, ident);
                }
                return Token::ident(ident);
            }
            a if is_digit(a) => {
                return Token::number(self.read_number());
            }
            a if a == '\0' => {
                tok = Token::eof();
            }
            a => {
                tok = Token::illegal(a);
            }
        }
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if let Some(ch) = self.input.next() {
            self.ch = ch;
        } else {
            self.ch = '\0';
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_white_space(&mut self) {
        while is_white_space(self.ch) {
            self.read_char();
        }
    }

    fn peak_char(&self) -> char {
        if let Some(ch) = self.input.clone().next() {
            ch
        } else {
            '\0'
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while is_letter(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }
        ident
    }

    fn read_number(&mut self) -> String {
        let mut num = String::new();
        while is_digit(self.ch) {
            num.push(self.ch);
            self.read_char();
        }
        num
    }
}

fn is_white_space(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
        	return true;
        } else {
        	return false;
        }
        
        10 == 10;
        10 != 9;
        "#;

        let tests: Vec<Token> = vec![
            Token::new(TokenType::Let, "let".into()),
            Token::new(TokenType::Ident, "five".into()),
            Token::new(TokenType::Assign, "=".into()),
            Token::new(TokenType::Int, "5".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::Let, "let".into()),
            Token::new(TokenType::Ident, "ten".into()),
            Token::new(TokenType::Assign, "=".into()),
            Token::new(TokenType::Int, "10".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::Let, "let".into()),
            Token::new(TokenType::Ident, "add".into()),
            Token::new(TokenType::Assign, "=".into()),
            Token::new(TokenType::Function, "fn".into()),
            Token::new(TokenType::LParen, "(".into()),
            Token::new(TokenType::Ident, "x".into()),
            Token::new(TokenType::Comma, ",".into()),
            Token::new(TokenType::Ident, "y".into()),
            Token::new(TokenType::RParen, ")".into()),
            Token::new(TokenType::LBrace, "{".into()),
            Token::new(TokenType::Ident, "x".into()),
            Token::new(TokenType::Plus, "+".into()),
            Token::new(TokenType::Ident, "y".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::RBrace, "}".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::Let, "let".into()),
            Token::new(TokenType::Ident, "result".into()),
            Token::new(TokenType::Assign, "=".into()),
            Token::new(TokenType::Ident, "add".into()),
            Token::new(TokenType::LParen, "(".into()),
            Token::new(TokenType::Ident, "five".into()),
            Token::new(TokenType::Comma, ",".into()),
            Token::new(TokenType::Ident, "ten".into()),
            Token::new(TokenType::RParen, ")".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::Bang, "!".into()),
            Token::new(TokenType::Minus, "-".into()),
            Token::new(TokenType::Slash, "/".into()),
            Token::new(TokenType::Asterisk, "*".into()),
            Token::new(TokenType::Int, "5".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::Int, "5".into()),
            Token::new(TokenType::LT, "<".into()),
            Token::new(TokenType::Int, "10".into()),
            Token::new(TokenType::GT, ">".into()),
            Token::new(TokenType::Int, "5".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::If, "if".into()),
            Token::new(TokenType::LParen, "(".into()),
            Token::new(TokenType::Int, "5".into()),
            Token::new(TokenType::LT, "<".into()),
            Token::new(TokenType::Int, "10".into()),
            Token::new(TokenType::RParen, ")".into()),
            Token::new(TokenType::LBrace, "{".into()),
            Token::new(TokenType::Return, "return".into()),
            Token::new(TokenType::True, "true".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::RBrace, "}".into()),
            Token::new(TokenType::Else, "else".into()),
            Token::new(TokenType::LBrace, "{".into()),
            Token::new(TokenType::Return, "return".into()),
            Token::new(TokenType::False, "false".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::RBrace, "}".into()),
            Token::new(TokenType::Int, "10".into()),
            Token::new(TokenType::Eq, "==".into()),
            Token::new(TokenType::Int, "10".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::Int, "10".into()),
            Token::new(TokenType::NEq, "!=".into()),
            Token::new(TokenType::Int, "9".into()),
            Token::new(TokenType::Semicolon, ";".into()),
            Token::new(TokenType::EOF, "".into()),
        ];

        let mut lexer = Lexer::new(input);

        for (idx, t) in tests.into_iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok, t, "Token {}", idx + 1);
        }
    }
}
