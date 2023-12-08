//! `source code` -lexer-> `tokens`
//! This transformation from source code to tokens, is called *lexical analysis*
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
            '+' => tok = Token::new(TokenType::PLUS, self.ch.into()),
            '-' => tok = Token::new(TokenType::MINUS, self.ch.into()),
            '*' => tok = Token::new(TokenType::ASTERISK, self.ch.into()),
            '/' => tok = Token::new(TokenType::SLASH, self.ch.into()),
            '<' => tok = Token::new(TokenType::LT, self.ch.into()),
            '>' => tok = Token::new(TokenType::GT, self.ch.into()),
            '(' => tok = Token::new(TokenType::LPAREN, self.ch.into()),
            ')' => tok = Token::new(TokenType::RPAREN, self.ch.into()),
            '{' => tok = Token::new(TokenType::LBRACE, self.ch.into()),
            '}' => tok = Token::new(TokenType::RBRACE, self.ch.into()),
            ',' => tok = Token::new(TokenType::COMMA, self.ch.into()),
            ';' => tok = Token::new(TokenType::SEMICOLON, self.ch.into()),
            '=' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    tok = Token::new(TokenType::EQ, "==".into());
                }
                _ => tok = Token::new(TokenType::ASSIGN, self.ch.into()),
            },
            '!' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    tok = Token::new(TokenType::NEQ, "!=".into());
                }
                _ => tok = Token::new(TokenType::BANG, self.ch.into()),
            },
            a if is_letter(a) => {
                // return directly, otherwise, line-63 will eat next char.
                let ident = self.read_identifier();
                if let Some(keyword_type) = TokenType::lookup_keyword(ident.as_str()) {
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
            Token::new(TokenType::LET, "let".into()),
            Token::new(TokenType::IDENT, "five".into()),
            Token::new(TokenType::ASSIGN, "=".into()),
            Token::new(TokenType::INT, "5".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::LET, "let".into()),
            Token::new(TokenType::IDENT, "ten".into()),
            Token::new(TokenType::ASSIGN, "=".into()),
            Token::new(TokenType::INT, "10".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::LET, "let".into()),
            Token::new(TokenType::IDENT, "add".into()),
            Token::new(TokenType::ASSIGN, "=".into()),
            Token::new(TokenType::FUNCTION, "fn".into()),
            Token::new(TokenType::LPAREN, "(".into()),
            Token::new(TokenType::IDENT, "x".into()), // here
            Token::new(TokenType::COMMA, ",".into()),
            Token::new(TokenType::IDENT, "y".into()),
            Token::new(TokenType::RPAREN, ")".into()),
            Token::new(TokenType::LBRACE, "{".into()),
            Token::new(TokenType::IDENT, "x".into()),
            Token::new(TokenType::PLUS, "+".into()),
            Token::new(TokenType::IDENT, "y".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::RBRACE, "}".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::LET, "let".into()),
            Token::new(TokenType::IDENT, "result".into()),
            Token::new(TokenType::ASSIGN, "=".into()),
            Token::new(TokenType::IDENT, "add".into()),
            Token::new(TokenType::LPAREN, "(".into()),
            Token::new(TokenType::IDENT, "five".into()),
            Token::new(TokenType::COMMA, ",".into()),
            Token::new(TokenType::IDENT, "ten".into()),
            Token::new(TokenType::RPAREN, ")".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::BANG, "!".into()),
            Token::new(TokenType::MINUS, "-".into()),
            Token::new(TokenType::SLASH, "/".into()),
            Token::new(TokenType::ASTERISK, "*".into()),
            Token::new(TokenType::INT, "5".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::INT, "5".into()),
            Token::new(TokenType::LT, "<".into()),
            Token::new(TokenType::INT, "10".into()),
            Token::new(TokenType::GT, ">".into()),
            Token::new(TokenType::INT, "5".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::IF, "if".into()),
            Token::new(TokenType::LPAREN, "(".into()),
            Token::new(TokenType::INT, "5".into()),
            Token::new(TokenType::LT, "<".into()),
            Token::new(TokenType::INT, "10".into()),
            Token::new(TokenType::RPAREN, ")".into()),
            Token::new(TokenType::LBRACE, "{".into()),
            Token::new(TokenType::RETURN, "return".into()),
            Token::new(TokenType::TRUE, "true".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::RBRACE, "}".into()),
            Token::new(TokenType::ELSE, "else".into()),
            Token::new(TokenType::LBRACE, "{".into()),
            Token::new(TokenType::RETURN, "return".into()),
            Token::new(TokenType::FALSE, "false".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::RBRACE, "}".into()),
            Token::new(TokenType::INT, "10".into()),
            Token::new(TokenType::EQ, "==".into()),
            Token::new(TokenType::INT, "10".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::INT, "10".into()),
            Token::new(TokenType::NEQ, "!=".into()),
            Token::new(TokenType::INT, "9".into()),
            Token::new(TokenType::SEMICOLON, ";".into()),
            Token::new(TokenType::EOF, "".into()),
        ];

        let mut lexer = Lexer::new(input);

        for (idx, t) in tests.into_iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok, t, "Token {}", idx + 1);
        }
    }
}
