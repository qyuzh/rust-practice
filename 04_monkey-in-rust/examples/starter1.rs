use std::io;

use monkey_in_rust::start;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands!");
    start(io::stdin(), io::stdout());
}
