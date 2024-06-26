//! `tokens` -parser-> `ast`
//! Top-down operator precedence parser or Pratt parser, that is a
//! recursive descent parser

use std::collections::HashMap;

use crate::ast::{
    BlockStatement, BooleanLiteral, CallExpression, Expression, ExpressionStatement,
    FunctionLiteral, Identifier, IfExpression, InfixExpression, IntegerLiteral, PrefixExpression,
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
            TokenType::Minus => Precedence::Sum,
            TokenType::Plus => Precedence::Sum,
            TokenType::Asterisk => Precedence::Product,
            TokenType::Slash => Precedence::Product,
            TokenType::LT => Precedence::LessGreater,
            TokenType::GT => Precedence::LessGreater,
            TokenType::Eq => Precedence::Equals,
            TokenType::NEq => Precedence::Equals,
            TokenType::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn<'a>>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut p = Self {
            l: lexer,
            cur_token: Token::eof(),
            peek_token: Token::eof(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        p.register_prefix(TokenType::Ident, Self::parse_identifier);
        p.register_prefix(TokenType::Int, Self::parse_integer_literal);
        p.register_prefix(TokenType::Bang, Self::parse_prefix_expression);
        p.register_prefix(TokenType::Minus, Self::parse_prefix_expression);
        p.register_prefix(TokenType::True, Self::parse_boolean_literal);
        p.register_prefix(TokenType::False, Self::parse_boolean_literal);
        p.register_prefix(TokenType::LParen, Self::parse_grouped_expression);
        p.register_prefix(TokenType::If, Self::parse_if_expression);

        p.register_infix(TokenType::Plus, Self::parse_infix_expression);
        p.register_infix(TokenType::Minus, Self::parse_infix_expression);
        p.register_infix(TokenType::Slash, Self::parse_infix_expression);
        p.register_infix(TokenType::Asterisk, Self::parse_infix_expression);
        p.register_infix(TokenType::Eq, Self::parse_infix_expression);
        p.register_infix(TokenType::NEq, Self::parse_infix_expression);
        p.register_infix(TokenType::LT, Self::parse_infix_expression);
        p.register_infix(TokenType::GT, Self::parse_infix_expression);
        p.register_infix(TokenType::LParen, Self::parse_call_expression);

        p.next_token();
        p.next_token();
        p
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.cur_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// parse `let <ident> = <expression>;`
    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token_let = self.cur_token.clone();

        if !self.expect_token(TokenType::Ident) {
            return None;
        }
        let token_ident = self.cur_token.clone();

        if !self.expect_token(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest).unwrap();

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(LetStatement {
            token: token_let,
            name: Identifier {
                value: token_ident.literal.clone(),
                token: token_ident,
            },
            value: exp,
        }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token_ret = self.cur_token.clone();
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest).unwrap();
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Box::new(ReturnStatement {
            token: token_ret,
            value: exp,
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;
        let stmt = ExpressionStatement { token, expression };
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(Box::new(stmt))
    }

    /// `1 + 2 * 3` => `(1 + (2 * 3))`
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        if let Some(f) = self.prefix_parse_fns.get(&self.cur_token.token_type) {
            let mut left = f(self);

            while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
                let token_type = self.peek_token.token_type;
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
        let cur_token = self.cur_token.clone();
        let operator = cur_token.literal.clone();
        self.next_token();
        Box::new(PrefixExpression {
            token: cur_token,
            operator,
            right: self.parse_expression(Precedence::Prefix).unwrap(),
        })
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let cur_token = self.cur_token.clone();
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
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_integer_literal(&mut self) -> Box<dyn Expression> {
        Box::new(IntegerLiteral {
            token: self.cur_token.clone(),
            value: atoi::atoi::<i64>(self.cur_token.literal.as_ref()).unwrap(),
        })
    }

    fn parse_boolean_literal(&mut self) -> Box<dyn Expression> {
        Box::new(BooleanLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        })
    }

    fn parse_grouped_expression(&mut self) -> Box<dyn Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest);
        if !self.expect_token(TokenType::RParen) {
            todo!()
        }
        exp.unwrap()
    }

    fn parse_if_expression(&mut self) -> Box<dyn Expression> {
        let cur_token = self.cur_token.clone();

        if !self.expect_token(TokenType::LParen) {
            todo!("expect TokenType::LParen in parse_if_expression");
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest).unwrap();

        if !self.expect_token(TokenType::RParen) {
            todo!("expect TokenType::RParen in parse_if_expression");
        }

        if !self.expect_token(TokenType::LBrace) {
            todo!("expect TokenType::LBrace in parse_if_expression");
        }

        let consequence = self.parse_block_statement();

        let mut alternative = None;
        if self.peek_token_is(TokenType::Else) {
            self.next_token();
            if !self.expect_token(TokenType::LBrace) {
                todo!("expect TokenType::LBrace in parse_if_expression else branch");
            }
            alternative = Some(self.parse_block_statement());
        }

        Box::new(IfExpression {
            token: cur_token,
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_function_literal(&mut self) -> Box<dyn Expression> {
        let cur_token = self.cur_token.clone();

        if !self.peek_token_is(TokenType::LParen) {
            todo!("expect TokenType::LParen in parse_function_literal");
        }

        let parameters = self.parse_function_parameters();

        if !self.peek_token_is(TokenType::RParen) {
            todo!("expect TokenType::RParen in parse_function_literal");
        }

        let body = self.parse_block_statement();

        Box::new(FunctionLiteral {
            token: cur_token,
            parameters,
            body,
        })
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut parameters = Vec::new();

        if self.peek_token_is(TokenType::RParen) {
            return parameters;
        }

        self.next_token();
        parameters.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token(); // eat comma
            self.next_token();
            parameters.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_token(TokenType::RParen) {
            todo!("expect TokenType::RParen in parse_function_parameters")
        }

        parameters
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let cur_token = self.cur_token.clone();
        let mut statements = Vec::new();

        self.next_token();
        while self.cur_token.token_type != TokenType::Eof {
            if self.cur_token_is(TokenType::RBrace) {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        BlockStatement {
            token: cur_token,
            statements,
        }
    }

    fn parse_call_expression(&mut self, f: Box<dyn Expression>) -> Box<dyn Expression> {
        Box::new(CallExpression {
            token: self.cur_token.clone(),
            function: f,
            arguments: self.parse_call_arguments(),
        })
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<dyn Expression>> {
        let mut args = Vec::new();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return args;
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest).unwrap());

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest).unwrap());
        }

        if !self.peek_token_is(TokenType::RParen) {
            todo!("expect TokenType::RParen in parse_call_arguments");
        }

        args
    }

    fn next_token(&mut self) {
        let token = self.l.next_token();
        let peek_token = std::mem::replace(&mut self.peek_token, token);
        let _ = std::mem::replace(&mut self.cur_token, peek_token);
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    /// if next token is t, then advance a token; otherwise do nothing
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
        Precedence::lookup(&self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::lookup(&self.cur_token.token_type)
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
            assert_eq!(TokenType::Let, ls.token.token_type);
        }

        println!("{}", program);
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
            assert_eq!(rs.token.token_type, TokenType::Return);
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

    #[test]
    fn display() {
        let mut lexer = Lexer::new("true;false");
        let mut p = Parser::new(lexer);
        let program = p.parse_program();
        println!("{program}");
    }
}
