use crate::ast::{Identifier, ReturnStatement, Statement};
use crate::ast::{LetStatement, Program};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut instance = Self {
            l: lexer,
            cur_token: None,
            peek_token: None,
        };
        instance.next_token();
        instance.next_token();
        instance
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        let token = self.l.next_token();
        if token.token_type == TokenType::EOF {
            self.peek_token = None;
        } else {
            self.peek_token = Some(token);
        }
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while let Some(_) = self.cur_token {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.as_ref().unwrap().token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => unimplemented!(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token_let = self.cur_token.clone();

        if !self.expect_token(TokenType::IDENT) {
            return None;
        }

        let ident = Identifier {
            token: self.cur_token.clone().unwrap(),
            value: self.cur_token.as_ref().unwrap().literal.clone(),
        };

        if !self.expect_token(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(LetStatement {
            token: token_let.unwrap(),
            name: ident,
            value: None,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let ret_token = self.cur_token.take();
        self.next_token();
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ReturnStatement {
            token: ret_token.unwrap(),
            value: None,
        }))
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.as_ref().unwrap().token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.as_ref().unwrap().token_type == t
    }

    fn expect_token(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{LetStatement, ReturnStatement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::TokenType;

    #[test]
    fn test_let_statement() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements"
        );

        let tests = ["x", "y", "foobar"];

        for t in tests.iter().zip(program.statements.iter()) {
            let p = t.1.as_any().downcast_ref::<LetStatement>().unwrap();
            assert_eq!(*(t.0), p.name.value);
            assert_eq!(TokenType::LET, p.token.token_type);
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements"
        );

        for t in program.statements.iter() {
            let t = t.as_any().downcast_ref::<ReturnStatement>().unwrap();
            assert_eq!(t.token.token_type, TokenType::RETURN);
        }
    }
}
