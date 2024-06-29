//! REPL stands for Read Eval Print Loop

use std::io;
use std::io::{BufRead, Write};

use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::parser::Parser;
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
            std_out.write_all(b"bye~");
            break;
        }

        let mut lexer = Lexer::new(sc.as_ref());

        let mut p = Parser::new(lexer);
        let program = p.parse_program();
        let evaluated = eval(&program);
        std_out.write_all(evaluated.inspect().as_ref());
        std_out.write_all(b"\n");
    }
}

pub fn lexer_starter(std_in: io::Stdin, mut std_out: io::Stdout) {
    loop {
        let _ = std_out.write_all(PROMPT.as_bytes());
        let _ = std_out.flush();

        let mut sc = String::new();
        let _ = std_in.read_line(&mut sc);

        if sc.starts_with(EXIT) {
            std_out.write_all(b"bye~");
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

pub fn parser_starter(std_in: io::Stdin, mut std_out: io::Stdout) {
    loop {
        let _ = std_out.write_all(PROMPT.as_bytes());
        let _ = std_out.flush();

        let mut sc = String::new();
        let _ = std_in.read_line(&mut sc);

        if sc.starts_with(EXIT) {
            std_out.write_all(b"bye~");
            break;
        }

        let mut lexer = Lexer::new(sc.as_ref());

        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        std_out.write_all(format!("{program}").as_ref());
    }
}
