use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    update(&root, 0);
    root
}

/// # Args
/// - great_sum, 比当前结点大的结点的和(不包括右子树)
///
/// # Returns
/// - 以node为根的子树的和
fn update(node: &Option<Rc<RefCell<TreeNode>>>, great_sum: i32) -> i32 {
    match node {
        None => 0,
        Some(node) => {
            let right_sum = update(&node.borrow().right, great_sum);
            let v = node.borrow_mut().val + right_sum;
            node.borrow_mut().val = v + great_sum;
            let left_sum = update(&node.borrow().left, node.borrow().val);
            v + left_sum
        }
    }
}
