use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if p.is_none() || q.is_none() || root.is_none() {
        return None;
    }

    let mut p_descendants = vec![];
    get_descendants(&root, &p.unwrap(), &mut p_descendants); // SAFETY: guard guarantee
    let mut q_descendants = vec![];
    get_descendants(&root, &q.unwrap(), &mut q_descendants); // SAFETY: guard guarantee

    for d in p_descendants.iter().rev() {
        // search
        for d2 in q_descendants.iter().rev() {
            if Rc::ptr_eq(d, d2) {
                return Some(d.clone());
            }
        }
    }

    None
}

type Node = Rc<RefCell<TreeNode>>;

fn get_descendants(root: &Option<Node>, p: &Node, descendants: &mut Vec<Node>) -> bool {
    match root {
        Some(node) => {
            if Rc::ptr_eq(node, p) {
                descendants.push(node.clone()); // fix: self is also descdendant
                true
            } else {
                descendants.push(node.clone());
                let left_founded = get_descendants(&node.borrow().left, p, descendants);
                let right_founded = get_descendants(&node.borrow().right, p, descendants);
                if !left_founded && !right_founded {
                    descendants.pop();
                }
                left_founded || right_founded
            }
        }
        None => false,
    }
}
