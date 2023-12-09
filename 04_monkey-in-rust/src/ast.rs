use std::any::Any;

use crate::token::Token;
use crate::{impl_node, impl_statement};

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

impl_node!(LetStatement, ReturnStatement, Identifier);
impl_statement!(LetStatement, ReturnStatement, Identifier);
