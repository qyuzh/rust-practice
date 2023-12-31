use std::any::Any;

use crate::ast::{
    BooleanLiteral, ExpressionStatement, IntegerLiteral, Node, PrefixExpression, Program, Statement,
};
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

    if let Some(t) = node.downcast_ref::<PrefixExpression>() {
        let right = eval(t.right.as_ref());
        return eval_prefix_expression(&t.operator, &right);
    }

    todo!()
}

fn eval_statements(stmts: &Vec<Box<dyn Statement>>) -> Box<dyn Object> {
    stmts.iter().fold(Box::new(Null), |_, t| eval(t.as_ref()))
}

fn eval_prefix_expression(operator: &str, right: &Box<dyn Object>) -> Box<dyn Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        _ => {
            todo!()
        }
    }
}

fn eval_bang_operator_expression(right: &Box<dyn Object>) -> Box<dyn Object> {
    if let Some(t) = right.as_any().downcast_ref::<Null>() {
        return Box::new(Boolean { value: true });
    }

    if let Some(t) = right.as_any().downcast_ref::<Boolean>() {
        return Box::new(Boolean { value: !t.value });
    }

    Box::new(Boolean { value: false }) // default false
}

#[cfg(test)]
mod test {
    use crate::ast::Program;
    use crate::evaluator::eval;
    use crate::lexer::Lexer;
    use crate::object::Boolean;
    use crate::parser::Parser;

    #[test]
    fn test_bang_operators() {
        let tests = [
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for (input, expected) in tests {
            let program = get_program(input);
            let evaluated = eval(&program);
            if let Some(t) = evaluated.as_any().downcast_ref::<Boolean>() {
                assert_eq!(t.value, expected);
            } else {
                assert!(false, "Expected Boolean");
            }
        }
    }

    fn get_program(s: &str) -> Program {
        Parser::new(Lexer::new(s)).parse_program()
    }
}
