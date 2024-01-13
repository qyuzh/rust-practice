use std::mem::size_of;

struct A {}

struct B {
    a: i16,
    b: i32,
    c: bool,
}

#[repr(C)]
struct C {
    a: i16,
    b: i32,
    c: bool,
}

fn main() {
    println!("{}", size_of::<bool>());

    let a1 = A {};
    let a2 = A {};
    let a3 = &a2;
    let b4 = a3;
    println!("{}", size_of::<A>());
    println!("{:p} {:p} {:p} {:p} {:p}", &a1, &a2, a3, &a3, b4);

    let b = B {
        a: 0,
        b: 0,
        c: false,
    };
    let p_ba = &b.a;
    let p_bb = &b.b;
    let p_bc = &b.c;

    println!("{}", size_of::<B>());
    println!("{:p} {:p} {:p} {:p}", &b, p_ba, p_bb, p_bc);

    let c = C {
        a: 0,
        b: 0,
        c: false,
    };
    let p_ca = &c.a;
    let p_cb = &c.b;
    let p_cc = &c.c;
    println!("{}", size_of::<C>());
    println!("{:p} {:p} {:p} {:p}", &c, p_ca, p_cb, p_cc);
}
