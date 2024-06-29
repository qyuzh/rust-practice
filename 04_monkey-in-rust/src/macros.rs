use std::any::{Any, TypeId};

pub const IDENT_SIZE: usize = 2;
pub const EMPTY_STR: &str = "";

pub fn is_primitive<T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<String>() == TypeId::of::<T>()
        || TypeId::of::<i64>() == TypeId::of::<T>()
        || TypeId::of::<bool>() == TypeId::of::<T>()
}

#[macro_export]
macro_rules! impl_node {
    ($($t: ty),+,) => {
        $(impl $crate::ast::Node for $t {
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
        $(impl $crate::ast::Statement for $t {
            fn statement_node(&self) {}
        })+
    };
}

#[macro_export]
macro_rules! impl_expression {
    ($($t: ty),+,) => {
        $(impl $crate::ast::Expression for $t {
            fn expression_node(&self) {}
        })+
    };
}

#[macro_export]
macro_rules! impl_display_for {
    ($tf:path: $($t: ty),+,) => {
        impl std::fmt::Display for Box<dyn $tf> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut w = 0;
                if let Some(width) = f.width() {
                    w = width;
                }
                let nw = w + $crate::macros::IDENT_SIZE;
                const EMPTY_STR: &str = $crate::macros::EMPTY_STR;
                $(if let Some(t) = self.as_any().downcast_ref::<$t>() {
                    return write!(f, "{t:nw$}");
                })+
                write!(f, "")
            }
        }
    };
}

/// (normal fields,option fields; vec fields)
#[macro_export]
macro_rules! impl_display_for_struct {
    ($ts:ty: ($($t: tt),*); ($($to: tt),*); ($($tv: tt), *)) => {
        impl std::fmt::Display for $ts {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

                let mut w = 0;
                if let Some(width) = f.width() {
                    w = width;
                }
                let nw = w + $crate::macros::IDENT_SIZE;
                const EMPTY_STR: &str = $crate::macros::EMPTY_STR;
                let mut s = String::new();

                // start {
                s.push_str(&format!("{} {{\n", stringify!($ts)));

                // fields
                // $({
                //     let end = if $crate::macros::is_primitive(&self.$t) { "\n" } else { "" };
                //     s.push_str(&format!("{EMPTY_STR:>nw$}{}: {:nw$}{end}", stringify!($t) ,self.$t));
                // })*

                $({
                    if $crate::macros::is_primitive(&self.$t) {
                        s.push_str(&format!("{EMPTY_STR:>nw$}{}: {}\n", stringify!($t) ,self.$t));
                    } else {
                        s.push_str(&format!("{EMPTY_STR:>nw$}{}: {:nw$}", stringify!($t) ,self.$t));
                    }
                })*

                // option fields
                $({
                    if self.$to.is_none() {
                        s.push_str(&format!("{EMPTY_STR:>nw$}{}: null\n", stringify!($to)));
                    } else {
                        let end = if $crate::macros::is_primitive(self.$to.as_ref().unwrap()) { "\n" } else { "" };
                        s.push_str(&format!("{EMPTY_STR:>nw$}{}: {:nw$}{end}", stringify!($to) ,self.$to.as_ref().unwrap()));
                    }
                })*

                // vec fields
                $({
                    s.push_str(&format!("{EMPTY_STR:>nw$}{}: [", stringify!($tv)));
                    let nww = nw + $crate::macros::IDENT_SIZE;
                    &self.$tv.iter().for_each(|v| { s.push_str(&format!("\n{EMPTY_STR:>nww$}{v:nw$}")); s.pop(); s.push_str(",") });
                    s.pop();
                    s.push_str(&format!("\n{EMPTY_STR:>nw$}]\n"));
                })*

                // end }
                s.push_str(&format!("{EMPTY_STR:>w$}}}\n"));
                // s.push_str(&format!("{EMPTY_STR:>w$}}} End of {}\n", stringify!($ts)));

                write!(f, "{s}")
            }
        }
    };
    ($ts:ty: $($t: tt),+,) => {
        $crate::impl_display_for_struct!($ts: ($($t),+); (); ());
    };
    ($ts:ty: o = $($t: tt),+,) => {
        $crate::impl_display_for_struct!($ts: (); ($($t),+); ());
    };
    ($ts:ty: v = $($t: tt),+,) => {
        $crate::impl_display_for_struct!($ts: (); (); ($($t),+));
    };
}

#[macro_export]
macro_rules! count {
    () => {
        0usize
    };
    ($_head: tt $($tail:tt)*) => {
      1usize + $crate::count!($($tail)*)
    };
}
