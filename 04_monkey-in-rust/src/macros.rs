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

#[macro_export]
macro_rules! impl_display_for {
    ($tf:path: $($t: ty),+,) => {
        impl std::fmt::Display for Box<dyn $tf> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $(if let Some(t) = self.as_any().downcast_ref::<$t>() {
                    return write!(f, "{t}");
                })+
                write!(f, "")
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display_for_struct {
    ($ts:ty: $($t: tt),+; $($to: tt),*) => {
        impl std::fmt::Display for $ts {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                const IDENT_SIZE: usize = 2;
                const EMPTY_STR: &str = "";
                use std::any::{Any, TypeId};
                fn is_primitive<T: ?Sized + Any>(_s: &T) -> bool {
                    TypeId::of::<String>() == TypeId::of::<T>() || TypeId::of::<i32>() == TypeId::of::<T>()
                }

                let mut w = 0;
                if let Some(width) = f.width() {
                    w = width;
                }
                let nw = w + IDENT_SIZE;
                let mut s = String::new();

                // start {
                s.push_str("{\n");

                // fields
                $({
                    let end = if is_primitive(&self.$t) { "\n" } else { "" };
                    s.push_str(&format!("{EMPTY_STR:>nw$}{}: {:nw$}{end}", stringify!($t) ,self.$t));
                })+

                $({
                    if self.$to.is_none() {
                        s.push_str(&format!("{EMPTY_STR:>nw$}{}: null\n", stringify!($to)));
                    } else {
                        let end = if is_primitive(self.$to.as_ref().unwrap()) { "\n" } else { "" };
                        s.push_str(&format!("{EMPTY_STR:>nw$}{}: {:nw$}{end}", stringify!($to) ,self.$to.as_ref().unwrap()));
                    }
                })*

                // end }
                s.push_str(&format!("{EMPTY_STR:>w$}}}\n"));

                write!(f, "{s}")
            }
        }
    };
}
