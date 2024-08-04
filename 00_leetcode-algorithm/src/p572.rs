use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    compare_some_root_with_sub_root(&root, &sub_root)
}

pub fn compare_some_root_with_sub_root(
    some_root: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if is_same_tree(some_root, sub_root) {
        return true;
    }
    if let Some(sr) = some_root {
        if compare_some_root_with_sub_root(&sr.borrow().left, sub_root)
            || compare_some_root_with_sub_root(&sr.borrow().right, sub_root)
        {
            return true;
        }
    }
    false
}

pub fn is_same_tree(
    root1: &Option<Rc<RefCell<TreeNode>>>,
    root2: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (root1, root2) {
        (None, None) => true,
        (Some(root1), Some(root2)) => {
            root1.borrow().val == root2.borrow().val
                && is_same_tree(&root1.borrow().left, &root2.borrow().left)
                && is_same_tree(&root1.borrow().right, &root2.borrow().right)
        }
        _ => false,
    }
}
