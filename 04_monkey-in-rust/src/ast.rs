use std::any::Any;

use crate::token::Token;
use crate::{
    impl_display_for, impl_display_for_struct, impl_expression, impl_node, impl_statement,
};

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

/// `let <identifier> = <expression>`
pub struct LetStatement {
    pub token: Token, // TokenType::Let
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

/// `return <expression>`
pub struct ReturnStatement {
    pub token: Token, // TokenType::Return
    pub value: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

pub struct ExpressionStatement {
    pub token: Token, // the first token of the expression
    pub expression: Box<dyn Expression>,
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
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

pub struct IfExpression {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

pub struct CallExpression {
    pub token: Token,                  // TokenType::LParen
    pub function: Box<dyn Expression>, // Identifier or FunctionLiteral
    pub arguments: Vec<Box<dyn Expression>>,
}
impl Node for Program {
    fn token_literal(&self) -> &str {
        "program"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl_node!(
    LetStatement,
    ReturnStatement,
    Identifier,
    ExpressionStatement,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
    BooleanLiteral,
    IfExpression,
    BlockStatement,
    FunctionLiteral,
    CallExpression,
);
impl_statement!(
    LetStatement,
    ReturnStatement,
    Identifier,
    ExpressionStatement,
    BlockStatement,
);
impl_expression!(
    Identifier,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
    BooleanLiteral,
    IfExpression,
    FunctionLiteral,
    CallExpression,
);

impl_display_for!(Statement: LetStatement, ExpressionStatement,);
impl_display_for!(Expression: PrefixExpression, InfixExpression, IntegerLiteral, BooleanLiteral,);
impl_display_for_struct!(Program: v = statements,);
impl_display_for_struct!(LetStatement: token, name, value,);
impl_display_for_struct!(Identifier: token, value,);
impl_display_for_struct!(ExpressionStatement: token, expression,);
impl_display_for_struct!(PrefixExpression: token, operator, right,);
impl_display_for_struct!(InfixExpression: token, operator, left, right,);
impl_display_for_struct!(IntegerLiteral: token, value,);
impl_display_for_struct!(BooleanLiteral: token, value,);
