use crate::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut orders = HashMap::new();

    dfs(&root, 0, &mut orders);

    let len = orders.len();
    let mut ans = vec![vec![]; len];

    orders.into_iter().for_each(|(idx, order)| {
        ans[idx] = if idx & 1 == 0 {
            order
        } else {
            order.into_iter().rev().collect()
        }
    });

    ans
}

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, orders: &mut HashMap<usize, Vec<i32>>) {
    if let Some(node) = root {
        orders
            .entry(depth)
            .and_modify(|v| v.push(node.borrow().val))
            .or_insert(vec![node.borrow().val]);
        dfs(&node.borrow().left, depth + 1, orders);
        dfs(&node.borrow().right, depth + 1, orders);
    }
}
