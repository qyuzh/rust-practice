use rand::Rng;
use std::cmp::Ordering;
use std::ptr;

const MAX_LEVEL: usize = 16;

struct Node {
    value: i32,
    ns: Vec<*mut Node>,
}

impl Node {
    fn new(value: i32, level: usize) -> Self {
        Node {
            value,
            ns: vec![ptr::null_mut(); level],
        }
    }
}

pub struct SkipList {
    head: *mut Node,
}

impl Default for SkipList {
    fn default() -> Self {
        Self::new()
    }
}

impl SkipList {
    pub fn new() -> Self {
        let head = Box::into_raw(Box::new(Node::new(i32::MIN, MAX_LEVEL)));
        SkipList { head }
    }

    fn find(&self, target: i32, ns: &mut [*mut Node; MAX_LEVEL]) {
        let mut cur = self.head;
        for i in (0..MAX_LEVEL).rev() {
            unsafe {
                while !(*cur).ns[i].is_null() && (*(*cur).ns[i]).value < target {
                    cur = (*cur).ns[i];
                }
                ns[i] = cur;
            }
        }
    }

    pub fn search(&self, target: i32) -> bool {
        let ns = &mut [ptr::null_mut(); MAX_LEVEL];
        self.find(target, ns);
        unsafe { !(*ns[0]).ns[0].is_null() && (*(*ns[0]).ns[0]).value == target }
    }

    pub fn add(&mut self, num: i32) {
        let ns = &mut [ptr::null_mut(); MAX_LEVEL];
        self.find(num, ns);
        let level = rand::rng().random_range(1..=MAX_LEVEL);
        let new_node = Box::into_raw(Box::new(Node::new(num, level)));
        #[allow(clippy::needless_range_loop)]
        for i in 0..level {
            unsafe {
                (*new_node).ns[i] = (*ns[i]).ns[i];
                (*ns[i]).ns[i] = new_node;
            }
        }
    }

    pub fn erase(&mut self, num: i32) -> bool {
        let ns = &mut [ptr::null_mut(); MAX_LEVEL];
        self.find(num, ns);
        let node = unsafe { (*ns[0]).ns[0] };
        if unsafe { node.is_null() || (*node).value != num } {
            return false;
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..MAX_LEVEL {
            unsafe {
                if (*ns[i]).ns[i] == node {
                    (*ns[i]).ns[i] = (*node).ns[i];
                }
            }
        }
        true
    }
}

impl Drop for SkipList {
    fn drop(&mut self) {
        let mut cur = self.head;
        while !unsafe { (*cur).ns[0].is_null() } {
            let next = unsafe { (*cur).ns[0] };
            unsafe {
                Box::from_raw(cur);
            }
            cur = next;
        }
        unsafe {
            Box::from_raw(cur);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skiplist() {
        let mut skiplist = SkipList::new();
        assert!(!skiplist.search(1)); // returns false
        skiplist.add(1);
        skiplist.add(2);
        skiplist.add(3);
        assert!(skiplist.search(1)); // returns true
        assert!(!skiplist.search(4)); // returns false
        skiplist.add(4);
        assert!(skiplist.search(4)); // returns true
        assert!(skiplist.erase(1)); // returns true
        assert!(!skiplist.search(1)); // returns false
        assert!(!skiplist.erase(1)); // returns false
        assert!(skiplist.search(2)); // returns true
        assert!(skiplist.search(3)); // returns true
        assert!(skiplist.search(4)); // returns true
        assert!(skiplist.erase(3)); // returns true
        assert!(!skiplist.search(3)); // returns false
    }

    #[test]
    fn test_skiplist_edge_cases() {
        let mut skiplist = SkipList::new();
        assert!(!skiplist.search(i32::MIN)); // returns false
        skiplist.add(i32::MIN);
        assert!(skiplist.search(i32::MIN)); // returns true
        assert!(skiplist.erase(i32::MIN)); // returns true
        assert!(!skiplist.search(i32::MIN)); // returns false

        assert!(!skiplist.search(i32::MAX)); // returns false
        skiplist.add(i32::MAX);
        assert!(skiplist.search(i32::MAX)); // returns true
        assert!(skiplist.erase(i32::MAX)); // returns true
        assert!(!skiplist.search(i32::MAX)); // returns false
    }
}
