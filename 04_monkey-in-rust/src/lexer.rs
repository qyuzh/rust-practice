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
            '=' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    tok = "==".into();
                }
                _ => {
                    tok = self.ch.into();
                }
            },
            '!' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    tok = "!=".into();
                }
                _ => {
                    tok = self.ch.into();
                }
            },
            '+' | '-' | '/' | '*' | '<' | '>' | ';' | ',' | '{' | '}' | '(' | ')' => {
                tok = self.ch.into();
            }
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
            a if a == '\0' as char => {
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
        ];

        let mut lexer = Lexer::new(input);

        for (idx, t) in tests.into_iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok, t, "Token {}", idx + 1);
        }
    }
}
