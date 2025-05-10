use std::ptr;

use rust_lang_learning::vec::MyVec;

/// NonNull::dangling can handle ZST but zero-sized offsets are no-ops
fn zst() {
    let p = ptr::NonNull::<()>::dangling();
    unsafe {
        #[allow(clippy::zst_offset)]
        let _p = p.as_ptr().add(1000); // zero-sized offsets are no-ops
        println!("{p:?} == {_p:?}");
        assert_eq!(p.as_ptr(), _p);
    }
    unsafe {
        ptr::write(p.as_ptr(), ());
    }
    #[allow(clippy::let_unit_value)]
    let t = unsafe { ptr::read(p.as_ptr()) };
    println!("{:?}", t);
}

/// ERROR
fn non_zst() {
    let p = ptr::NonNull::<i32>::dangling();
    unsafe {
        ptr::write(p.as_ptr(), 1);
    }
    let t = unsafe { ptr::read(p.as_ptr()) };
    println!("{t}");
}

fn my_vec() {
    let mut t = MyVec::new();
    t.push(1);
    t.push(2);
    t.push(3);
    t.pop();
    for x in t.into_iter() {
        print!("{x} ");
    }
    println!();
}

fn main() {
    zst();
    non_zst();
    my_vec();

    let p = "123".to_string();
    for x in p.chars() {
        if x == '1' {
            println!("{x}");
        }
    }
}
