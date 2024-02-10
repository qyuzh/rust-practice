use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut seq = vec![];
    inorder(&root, &mut seq);
    seq
}

fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, seq: &mut Vec<i32>) {
    match root {
        Some(node) => {
            inorder(&node.borrow().left, seq);
            seq.push(node.borrow().val);
            inorder(&node.borrow().right, seq);
        }
        None => {}
    }
}
