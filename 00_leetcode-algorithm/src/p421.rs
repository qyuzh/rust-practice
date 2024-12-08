use std::ptr;

/// HashSet in O(cn)/O(cn)
pub fn find_maximum_xor_v0(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();
    let high_bit = 31 - max.leading_zeros() as i32; // max could be 0

    let mut ans = 0;
    let mut mask = 0;
    let mut seen = std::collections::HashSet::new();
    for i in (0..=high_bit).rev() {
        seen.clear();
        mask |= 1 << i;
        let new_ans = ans | (1 << i);
        for &x in nums.iter() {
            let x = x & mask;
            if seen.contains(&(new_ans ^ x)) {
                ans = new_ans;
                break;
            }
            seen.insert(x);
        }
    }
    ans
}

/// Trie in O(cn)/O(cn) in which c = 32.
pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let tire = Trie::new();
    for &x in nums.iter() {
        tire.insert(x);
        ans = ans.max(tire.lookup(x));
    }
    ans
}

struct TrieNode {
    left: *mut TrieNode,  // 1
    right: *mut TrieNode, // 0
    end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            end: false,
        }
    }
}

struct Trie {
    head: *mut TrieNode,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            head: Box::into_raw(Box::new(TrieNode::new())),
        }
    }

    pub fn insert(&self, n: i32) {
        self.insert_node(self.head, n);
    }

    fn insert_node(&self, node: *mut TrieNode, mut n: i32) {
        let mut node = node;
        for _ in (0..32).rev() {
            unsafe {
                if (n & (1 << 31)) == (1 << 31) {
                    if (*node).left.is_null() {
                        (*node).left = Box::into_raw(Box::new(TrieNode::new()));
                    }
                    node = (*node).left;
                } else {
                    if (*node).right.is_null() {
                        (*node).right = Box::into_raw(Box::new(TrieNode::new()));
                    }
                    node = (*node).right;
                }
            }
            n <<= 1;
            if n == 0 {
                break;
            }
        }
        unsafe { (*node).end = true }
    }

    pub fn lookup(&self, n: i32) -> i32 {
        self.lookup_node(self.head, n)
    }

    fn lookup_node(&self, node: *mut TrieNode, mut n: i32) -> i32 {
        let tn = n;
        let mut c = 0;
        let mut node = node;
        let mut ans = 0;
        for i in (0..32).rev() {
            unsafe {
                if (n & (1 << 31)) == (1 << 31) {
                    if !(*node).right.is_null() {
                        node = (*node).right;
                    } else if !(*node).left.is_null() {
                        node = (*node).left;
                        c |= 1 << i;
                    } else {
                        break;
                    }
                } else if !(*node).left.is_null() {
                    node = (*node).left;
                    c |= 1 << i;
                } else if !(*node).right.is_null() {
                    node = (*node).right;
                } else {
                    break;
                }
                n <<= 1;
                if (*node).end {
                    ans = ans.max(c ^ tn);
                }
            }
        }
        ans
    }

    fn drop(node: *mut TrieNode) {
        if node.is_null() {
            return;
        }
        unsafe {
            Self::drop((*node).right);
            Self::drop((*node).left);
        }
        let _ = unsafe { Box::from_raw(node) };
    }
}

impl Drop for Trie {
    fn drop(&mut self) {
        Self::drop(self.head);
    }
}

#[cfg(test)]
mod test {
    use crate::p421::find_maximum_xor;

    #[test]
    fn test_find_maximum_xor() {
        assert_eq!(
            find_maximum_xor(vec![3, 10, 5, 25, 2, 8]),
            28,
            "find_maximum_xor 1"
        );
        assert_eq!(
            find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127,
            "find_maximum_xor 2"
        );
    }
}
