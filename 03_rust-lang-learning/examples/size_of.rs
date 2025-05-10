use std::mem::{size_of, size_of_val, MaybeUninit};

// assume on 64-bit platform
fn main() {
    let t = ['1', '2', '3'];
    let p = &t[..2];
    let p1 = &t[..];
    let reference = &p;

    println!("{}", size_of::<char>()); // 4 bytes
    println!("{}", size_of::<[char; 3]>()); // 12 bytes
    println!("{}", size_of_val(&t)); // 12 bytes
    println!("{}", size_of_val(&p)); // 16 bytes, slice
    println!("{}", size_of_val(&p1)); // 16 bytes, slice
    println!("{}", size_of_val(&reference)); // 8 bytes, reference
    println!("{}", size_of::<Option<i128>>()); // 24 bytes = 16 bytes + 8 bytes discriminant
    println!("{}", size_of::<Option<&i128>>()); // 8 bytes, optimization for reference, no discriminant

    // 2025-04-13
    println!("{}", size_of::<String>());
    println!("{}", size_of::<MaybeUninit<String>>());
}
