use crate::ast::{BooleanLiteral, ExpressionStatement, IntegerLiteral, Node, Program, Statement};
use crate::object::{Boolean, Integer, Null, Object};

pub fn eval<T: Node + ?Sized>(node: &T) -> Box<dyn Object> {
    let node = node.as_any();

    if let Some(t) = node.downcast_ref::<Program>() {
        return eval_statements(t.statements.as_ref());
    }

    if let Some(t) = node.downcast_ref::<ExpressionStatement>() {
        return eval(t.expression.as_ref());
    }

    if let Some(t) = node.downcast_ref::<IntegerLiteral>() {
        return Box::new(Integer { value: t.value });
    }

    if let Some(t) = node.downcast_ref::<BooleanLiteral>() {
        return Box::new(Boolean { value: t.value });
    }

    todo!()
}

fn eval_statements(stmts: &Vec<Box<dyn Statement>>) -> Box<dyn Object> {
    stmts
        .iter()
        .fold(Box::new(Null {}), |_, t| eval(t.as_ref()))
}
