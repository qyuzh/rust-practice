#![allow(unused)]

fn mutate(i: &mut u32) -> &mut u32 {
    *i += 1;
    i
}

// compiles
fn mutate_twice(i: &mut u32) -> &mut u32 {
    mutate(i);
    mutate(i);
    *i += 1;
    i
}

// compiles
fn reborrow() {
    let mut num = 32_u32;
    let a = &mut num;
    let b: &mut _ = a; // reborrow
    *b += 1; // out of scope
    *a += 1;
}

// does not compile
fn mut_move() {
    let mut num = 32_u32;
    let a = &mut num;
    let b = a; // move
    *b += 1;
    *a += 1;
}

fn main() {}
