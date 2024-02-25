use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if p.is_none() || q.is_none() {
        return None;
    }

    dfs(&root, p.as_ref().unwrap(), q.as_ref().unwrap())
}

fn dfs(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: &Rc<RefCell<TreeNode>>,
    q: &Rc<RefCell<TreeNode>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let p_val = p.borrow().val;
        let q_val = q.borrow().val;
        let r_val = node.borrow().val;

        if r_val < p_val.min(q_val) {
            return dfs(&node.borrow().right, p, q);
        }
        if r_val > p_val.max(q_val) {
            return dfs(&node.borrow().left, p, q);
        }
        return Some(node.clone());
    }
    None
}
