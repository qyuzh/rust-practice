use std::any::Any;

use crate::token::Token;
use crate::{impl_expression, impl_node, impl_statement};

pub trait Node {
    fn token_literal(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

/// A program consists of some statements
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    fn token_literal(&self) -> &str {
        return if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            ""
        };
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Box<dyn Expression>>,
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl_node!(
    LetStatement,
    ReturnStatement,
    Identifier,
    ExpressionStatement,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
);
impl_statement!(
    LetStatement,
    ReturnStatement,
    Identifier,
    ExpressionStatement,
);
impl_expression!(
    Identifier,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
);
