use std::mem::{size_of, size_of_val};

struct Dog {}

trait Behavior {
    fn bark(&self);
}

impl Behavior for Dog {
    fn bark(&self) {
        println!("bark");
    }
}

fn test(animal: &dyn Behavior) {
    println!("{}", size_of_val(&animal)); // 16 bytes
    animal.bark();
}

fn main() {
    let dog = Dog {};
    println!("{}", size_of_val(&dog)); // 0 bytes
    println!("{}", size_of::<&dyn Behavior>()); // 16 bytes
    test(&dog);
}
