pub struct MyStack {
    inner: std::collections::LinkedList<i32>,
}

/// `&self` means the method takes an immutable reference.
/// If you need a mutable reference, change it to `&mut self` instead.
impl MyStack {
    fn new() -> Self {
        Self {
            inner: std::collections::LinkedList::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.inner.push_back(x)
    }

    fn pop(&mut self) -> i32 {
        self.inner.pop_back().unwrap()
    }

    fn top(&self) -> i32 {
        *self.inner.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}
