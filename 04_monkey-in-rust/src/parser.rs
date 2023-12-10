use std::collections::HashMap;

use crate::ast::{
    Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, PrefixExpression,
    ReturnStatement, Statement,
};
use crate::ast::{LetStatement, Program};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Box<dyn Expression>;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Box<dyn Expression>) -> Box<dyn Expression>;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest = 1,  // discriminant starts from 1
    Equals,      // ==
    LessGreater, // < or >
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // fn(X)
}

impl Precedence {
    fn lookup(token_type: &TokenType) -> Precedence {
        match token_type {
            TokenType::MINUS => Precedence::Sum,
            TokenType::PLUS => Precedence::Sum,
            TokenType::ASTERISK => Precedence::Product,
            TokenType::SLASH => Precedence::Product,
            TokenType::LT => Precedence::LessGreater,
            TokenType::GT => Precedence::LessGreater,
            TokenType::EQ => Precedence::Equals,
            TokenType::NEQ => Precedence::Equals,
            _ => Precedence::Lowest,
        }
    }
}

struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Self {
            l: lexer,
            cur_token: None,
            peek_token: None,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        p.register_prefix(TokenType::IDENT, Self::parse_identifier);
        p.register_prefix(TokenType::INT, Self::parse_integer_literal);
        p.register_prefix(TokenType::BANG, Self::parse_prefix_expression);
        p.register_prefix(TokenType::MINUS, Self::parse_prefix_expression);

        p.register_infix(TokenType::PLUS, Self::parse_infix_expression);
        p.register_infix(TokenType::MINUS, Self::parse_infix_expression);
        p.register_infix(TokenType::SLASH, Self::parse_infix_expression);
        p.register_infix(TokenType::ASTERISK, Self::parse_infix_expression);
        p.register_infix(TokenType::EQ, Self::parse_infix_expression);
        p.register_infix(TokenType::NEQ, Self::parse_infix_expression);
        p.register_infix(TokenType::LT, Self::parse_infix_expression);
        p.register_infix(TokenType::GT, Self::parse_infix_expression);

        p.next_token();
        p.next_token();
        p
    }

    fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while let Some(_) = self.cur_token {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.as_ref().unwrap().token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token_let = self.cur_token.clone().unwrap();

        if !self.expect_token(TokenType::IDENT) {
            return None;
        }
        let token_ident = self.cur_token.clone().unwrap();

        if !self.expect_token(TokenType::ASSIGN) {
            return None;
        }

        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(LetStatement {
            token: token_let,
            name: Identifier {
                value: token_ident.literal.clone(),
                token: token_ident,
            },
            value: None,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token_ret = self.cur_token.take().unwrap();
        self.next_token();
        while !self.cur_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ReturnStatement {
            token: token_ret,
            value: None,
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ExpressionStatement {
            token: self.cur_token.clone().unwrap(),
            expression: self.parse_expression(Precedence::Lowest).unwrap(),
        };
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        if let Some(f) = self
            .prefix_parse_fns
            .get(&self.cur_token.as_ref().unwrap().token_type)
        {
            let mut left = f(self);

            while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
                let token_type = self.peek_token.as_ref().unwrap().token_type;

                if !self.infix_parse_fns.contains_key(&token_type) {
                    return None;
                }

                self.next_token();
                left = self.infix_parse_fns.get(&token_type).unwrap()(self, left);
            }

            Some(left)
        } else {
            None
        }
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        let cur_token = self.cur_token.clone().unwrap();
        let operator = cur_token.literal.clone();
        self.next_token();
        Box::new(PrefixExpression {
            token: cur_token,
            operator,
            right: self.parse_expression(Precedence::Prefix).unwrap(),
        })
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let cur_token = self.cur_token.clone().unwrap();
        let operator = cur_token.literal.clone();

        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence).unwrap();

        Box::new(InfixExpression {
            token: cur_token,
            operator,
            left,
            right,
        })
    }

    fn parse_identifier(&mut self) -> Box<dyn Expression> {
        Box::new(Identifier {
            token: self.cur_token.clone().unwrap(),
            value: self.cur_token.as_ref().unwrap().literal.clone(),
        })
    }

    fn parse_integer_literal(&mut self) -> Box<dyn Expression> {
        Box::new(IntegerLiteral {
            token: self.cur_token.clone().unwrap(),
            value: atoi::atoi::<i64>(self.cur_token.as_ref().unwrap().literal.as_ref()).unwrap(),
        })
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

    fn register_prefix(&mut self, token_type: TokenType, f: PrefixParseFn<'a>) {
        self.prefix_parse_fns.insert(token_type, f);
    }

    fn register_infix(&mut self, token_type: TokenType, f: InfixParseFn<'a>) {
        self.infix_parse_fns.insert(token_type, f);
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::lookup(&self.peek_token.as_ref().unwrap().token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::lookup(&self.cur_token.as_ref().unwrap().token_type)
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{
        ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
        PrefixExpression, ReturnStatement,
    };
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::token::TokenType;

    #[test]
    fn test_parsing_let_statement() {
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
            let ls = t.1.as_any().downcast_ref::<LetStatement>().unwrap();
            assert_eq!(*(t.0), ls.name.value);
            assert_eq!(TokenType::LET, ls.token.token_type);
        }
    }

    #[test]
    fn test_parsing_return_statement() {
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
            let rs = t.as_any().downcast_ref::<ReturnStatement>().unwrap();
            assert_eq!(rs.token.token_type, TokenType::RETURN);
        }
    }

    #[test]
    fn test_parsing_identifier_expression() {
        let input = r#"
        foobar;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statements"
        );

        let tests = ["foobar"];

        for t in program.statements.iter().zip(tests) {
            let es = t.0.as_any().downcast_ref::<ExpressionStatement>().unwrap();
            let ident = es.expression.as_any().downcast_ref::<Identifier>().unwrap();
            assert_eq!(ident.value, t.1);
        }
    }

    #[test]
    fn test_parsing_integer_literal() {
        let input = r#"
        5;
        123;
        "#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        assert_eq!(
            program.statements.len(),
            2,
            "program.statements does not contain 2 statements"
        );

        let tests = [5, 123];

        for t in program.statements.iter().zip(tests) {
            let es = t.0.as_any().downcast_ref::<ExpressionStatement>().unwrap();
            let il = es
                .expression
                .as_any()
                .downcast_ref::<IntegerLiteral>()
                .unwrap();
            assert_eq!(il.value, t.1);
        }
    }

    #[test]
    fn test_parsing_prefix_expression() {
        // (input, operator, integer value)
        let tests = [("!5;", "!", 5), ("-123;", "-", 123)];

        for (input, ops, val) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            let pe = program.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .as_any()
                .downcast_ref::<PrefixExpression>()
                .unwrap();
            assert_eq!(pe.operator, ops);
            assert_eq!(
                pe.right
                    .as_any()
                    .downcast_ref::<IntegerLiteral>()
                    .unwrap()
                    .value,
                val
            );
        }
    }

    #[test]
    fn test_parsing_infix_expression() {
        let tests = [
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (input, left, ops, right) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            let es = program.statements[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap();
            let ie = es
                .expression
                .as_any()
                .downcast_ref::<InfixExpression>()
                .unwrap();
            assert_eq!(
                ie.left
                    .as_any()
                    .downcast_ref::<IntegerLiteral>()
                    .unwrap()
                    .value,
                5
            );
            assert_eq!(ie.operator, ops);
            assert_eq!(
                ie.right
                    .as_any()
                    .downcast_ref::<IntegerLiteral>()
                    .unwrap()
                    .value,
                5
            );
        }
    }
}
