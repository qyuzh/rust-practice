#![allow(unused)]

use std::fmt::{format, Formatter};

struct Node<T> {
    val: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn raw_ptr(val: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            val,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }))
    }
}

struct FrontMiddleBackQueue {
    head: *mut Node<i32>,
    middle: *mut Node<i32>,
    last: *mut Node<i32>,
    len: usize,
}

impl FrontMiddleBackQueue {
    pub fn new() -> Self {
        Self {
            head: std::ptr::null_mut(),
            middle: std::ptr::null_mut(),
            last: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, val: i32) {
        if self.len == 0 {
            self.first_insert(val);
            return;
        }

        let node = Node::raw_ptr(val);

        unsafe {
            // 1 -> 2 1
            // m    m
            (*node).next = self.head; // 2 -> 1
            (*self.head).prev = node; // 2 <- 1
            if self.len % 2 == 1 {
                self.middle = (*self.middle).prev
            }
        }

        self.head = node;
        self.len += 1;
    }

    pub fn push_middle(&mut self, val: i32) {
        if self.len == 0 {
            self.first_insert(val);
            return;
        }

        let node = Node::raw_ptr(val);

        unsafe {
            // 1 -> 2 1
            // m    m
            if self.len == 1 {
                (*node).next = self.middle; // 2 -> 1
                (*self.middle).prev = node; // 2 <- 1
                self.head = node; // -> 2
            } else if self.len % 2 == 1 {
                // 1 2 3 -> 1 4 2 3
                //   m        m
                (*node).prev = (*self.middle).prev; // 1 <- 4
                (*(*self.middle).prev).next = node; // 1 -> 4
                (*node).next = self.middle; // 4 -> 2
                (*self.middle).prev = node; // 4 <- 2
            } else {
                // 1 2 -> 1 3 2
                // m        m
                (*node).prev = self.middle; // 1 <- 3
                (*(*self.middle).next).prev = node; // 3 <- 2
                (*node).next = (*self.middle).next; // 3 -> 2
                (*self.middle).next = node; // 1 -> 3
            }
        }

        self.middle = node;
        self.len += 1;
    }

    pub fn push_back(&mut self, val: i32) {
        if self.len == 0 {
            self.first_insert(val);
            return;
        }

        let node = Node::raw_ptr(val);

        unsafe {
            // 1 2 -> 1 2 3
            // m        m
            (*self.last).next = node;
            (*node).prev = self.last;
            if self.len % 2 == 0 {
                self.middle = (*self.middle).next;
            }
        }

        self.last = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> i32 {
        if self.len == 0 {
            return -1;
        }

        let val = unsafe { (*self.head).val };

        if self.len == 1 {
            self.reset();
            return val;
        }

        let p = self.head;

        // 1 2 3 4 -> 2 3 4
        unsafe {
            self.head = (*self.head).next;
            (*self.head).prev = std::ptr::null_mut();
            if self.len % 2 == 0 {
                self.middle = (*self.middle).next;
            }
        }

        let _ = unsafe { Box::from_raw(p) };
        self.len -= 1;

        val
    }

    pub fn pop_middle(&mut self) -> i32 {
        if self.len == 0 {
            return -1;
        }

        let val = unsafe { (*self.middle).val };

        if self.len == 1 {
            self.reset();
            return val;
        }

        let p = self.middle;

        unsafe {
            // 1 2 -> 2
            // m      m
            if self.len == 2 {
                self.middle = (*self.middle).next;
                self.head = self.middle;
            } else if self.len % 2 == 1 {
                // 1 2 3 -> 1 3
                //   m   -> m
                (*(*self.middle).prev).next = (*self.middle).next; // 1 -> 3
                (*(*self.middle).next).prev = (*self.middle).prev; // 1 <- 3
                self.middle = (*self.middle).prev; // -> 1
            } else {
                // 1 2 3 4 -> 1 3 4
                //   m          m
                (*(*self.middle).prev).next = (*self.middle).next; // 1 -> 3
                (*(*self.middle).next).prev = (*self.middle).prev; // 1 <- 3
                self.middle = (*self.middle).next; // -> 3
            }
        }

        let _ = unsafe { Box::from_raw(p) }; // drop p
        self.len -= 1;

        val
    }

    pub fn pop_back(&mut self) -> i32 {
        if self.len == 0 {
            return -1;
        }

        let val = unsafe { (*self.last).val };

        if self.len == 1 {
            self.reset();
            return val;
        }

        let p = self.last;

        // 1 2 3 4 -> 1 2 3
        unsafe {
            self.last = (*self.last).prev;
            (*self.last).next = std::ptr::null_mut();
            if self.len % 2 == 1 {
                self.middle = (*self.middle).prev;
            }
        }

        let _ = unsafe { Box::from_raw(p) };
        self.len -= 1;

        val
    }

    fn first_insert(&mut self, val: i32) {
        let node = Node::raw_ptr(val);
        self.head = node;
        self.middle = node;
        self.last = node;
        self.len += 1;
    }

    fn reset(&mut self) {
        let _ = unsafe { Box::from_raw(self.head) };
        self.head = std::ptr::null_mut();
        self.middle = std::ptr::null_mut();
        self.last = std::ptr::null_mut();
        self.len = 0;
    }
}

impl std::fmt::Display for FrontMiddleBackQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("(len = {})", self.len));
        if self.len != 0 {
            let mut p = self.head;
            while !p.is_null() {
                unsafe {
                    s.push_str(&format!(" {}", (*p).val));
                    p = (*p).next;
                }
            }
        }
        write!(f, "{s}")
    }
}

impl Drop for FrontMiddleBackQueue {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }
        let mut p = self.head;
        while !p.is_null() {
            unsafe {
                let t = (*p).next;
                let _ = Box::from_raw(p);
                p = t;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::p1670::FrontMiddleBackQueue;

    #[test]
    fn test_case_1() {
        let mut t = FrontMiddleBackQueue::new();
        t.push_front(1);
        t.push_back(2);
        t.push_middle(3);
        t.push_middle(4);

        assert_eq!(t.pop_front(), 1);
        assert_eq!(t.pop_middle(), 3);
        assert_eq!(t.pop_middle(), 4);
        assert_eq!(t.pop_back(), 2);
        assert_eq!(t.pop_front(), -1);
    }

    #[test]
    fn test_case_2() {
        let mut t = FrontMiddleBackQueue::new();
        t.push_front(1);
        t.push_front(2);
        t.push_front(3);
        t.push_front(4);

        assert_eq!(t.pop_back(), 1);
        assert_eq!(t.pop_back(), 2);
        assert_eq!(t.pop_back(), 3);
        assert_eq!(t.pop_back(), 4);
    }
}
