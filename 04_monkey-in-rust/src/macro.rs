#[macro_export]
macro_rules! impl_node {
    ($($t: ty),+,) => {
        $(impl crate::ast::Node for $t {
            fn token_literal(&self) -> &str {
                self.token.literal.as_ref()
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        })+
    };
}

#[macro_export]
macro_rules! impl_statement {
    ($($t: ty),+,) => {
        $(impl crate::ast::Statement for $t {
            fn statement_node(&self) {}
        })+
    };
}

#[macro_export]
macro_rules! impl_expression {
    ($($t: ty),+,) => {
        $(impl crate::ast::Expression for $t {
            fn expression_node(&self) {}
        })+
    };
}
