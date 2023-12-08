#![allow(unused)]

#[macro_use]
extern crate lazy_static;

pub use repl::start;

mod lexer;
mod repl;
mod token;
