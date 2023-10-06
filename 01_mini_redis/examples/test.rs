#[derive(Copy)]
struct CanDrop {}

impl Clone for CanDrop {
    fn clone(&self) -> Self {
        todo!()
    }
}



fn main() {
    println!("hello, world!");
}