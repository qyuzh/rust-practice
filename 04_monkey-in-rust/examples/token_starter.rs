use std::io;

use monkey_in_rust::lexer_starter;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands!");
    lexer_starter(io::stdin(), io::stdout());
}
