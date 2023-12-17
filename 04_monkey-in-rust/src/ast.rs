use std::any::Any;
use std::fmt;
use std::fmt::Formatter;

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

impl_display_for!(Statement: LetStatement,);

pub trait Expression: Node {
    fn expression_node(&self);
}

impl fmt::Display for Box<dyn Expression> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<expression>\n")
    }
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

impl fmt::Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.statements
            .iter()
            .for_each(|v| s.push_str(&format!("{v}")));
        write!(f, "{s}")
    }
}

/// `let <identifier> = <expression>`
pub struct LetStatement {
    pub token: Token, // TokenType::Let
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl_display_for_struct!(LetStatement: token: value);

/// `return <expression>`
pub struct ReturnStatement {
    pub token: Token, // TokenType::Return
    pub value: Option<Box<dyn Expression>>,
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

pub struct Boolean {
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

impl_node!(
    LetStatement,
    ReturnStatement,
    Identifier,
    ExpressionStatement,
    IntegerLiteral,
    PrefixExpression,
    InfixExpression,
    Boolean,
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
    Boolean,
    IfExpression,
    FunctionLiteral,
    CallExpression,
);
