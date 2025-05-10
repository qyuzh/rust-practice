pub mod arc;
pub mod arc_swap;
pub mod spin_lock;
pub mod vec;

extern "C" {
    pub fn hello();
}

#[test]
pub fn understand_linked_list_in_std() {
    let mut ll = std::collections::LinkedList::new();
    ll.push_front(1);
    ll.push_front(3);
    ll.push_front(2);

    for &v in ll.iter() {
        println!("{v}");
    }
}
