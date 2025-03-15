use std::cell::RefCell;
use std::ops::{Bound, RangeBounds};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, bounds: (Bound<i32>, Bound<i32>)) -> bool {
        if let Some(node) = root {
            return bounds.contains(&node.borrow().val)
                && helper(
                    &node.borrow().left,
                    (bounds.0, Bound::Excluded(node.borrow().val)),
                )
                && helper(
                    &node.borrow().right,
                    (Bound::Excluded(node.borrow().val), bounds.1),
                );
        }
        true
    }
    helper(&root, (Bound::Unbounded, Bound::Unbounded))
}
