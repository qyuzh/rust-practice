//! REPL stands for Read Eval Print Loop

use std::io;
use std::io::{BufRead, Write};

use crate::lexer::Lexer;
use crate::token::Token;

const PROMPT: &str = ">> ";
const EXIT: &str = ".exit";

pub fn start(std_in: io::Stdin, mut std_out: io::Stdout) {
    loop {
        let _ = std_out.write_all(PROMPT.as_bytes());
        let _ = std_out.flush();

        let mut sc = String::new();
        let _ = std_in.read_line(&mut sc);

        if sc.starts_with(EXIT) {
            std_out.write_all(format!("bye~").as_ref());
            break;
        }

        let mut lexer = Lexer::new(sc.as_ref());
        let mut tok = lexer.next_token();
        std_out.write_all(format!("{tok:?}\n").as_ref());
        while tok != Token::eof() {
            tok = lexer.next_token();
            std_out.write_all(format!("{tok:?}\n").as_ref());
        }
    }
}
