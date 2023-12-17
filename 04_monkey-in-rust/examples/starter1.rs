use std::fmt;
use std::fmt::{Display, Formatter};

const IDENT_SIZE: usize = 2;
const EMPTY_STR: &str = "";

struct A(i32);

impl fmt::Display for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.0)
    }
}

struct B {
    a: A,
}

impl fmt::Display for B {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut w = 0;
        if let Some(width) = f.width() {
            w = width;
        }
        let nw = w + IDENT_SIZE;
        let mut s = String::new();

        s.push_str("{\n");

        s.push_str(&format!("{EMPTY_STR:>nw$}a: {:nw$}", self.a));

        s.push_str(&format!("{EMPTY_STR:>w$}}}\n"));

        write!(f, "{s}")
    }
}

struct C {
    b: B,
    a: A,
    bb: B,
}

impl fmt::Display for C {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut w = 0;
        if let Some(width) = f.width() {
            w = width;
        }
        let nw = w + IDENT_SIZE;
        let mut s = String::new();

        // start {
        s.push_str("{\n");

        // fields
        s.push_str(&format!("{EMPTY_STR:>nw$}b: {:nw$}", self.b));
        s.push_str(&format!("{EMPTY_STR:>nw$}a: {:nw$}", self.a));
        s.push_str(&format!("{EMPTY_STR:>nw$}bb: {:nw$}", self.bb));

        // end }
        s.push_str(&format!("{EMPTY_STR:>w$}}}\n"));

        write!(f, "{s}")
    }
}

fn main() {
    // println!(
    //     "Hello {}! This is the Monkey programming language!",
    //     whoami::username()
    // );
    // println!("Feel free to type in commands!");
    // start(io::stdin(), io::stdout());
    let c = C {
        b: B { a: A(0) },
        a: A(1),
        bb: B { a: A(2) },
    };
    println!("{c}");
}
