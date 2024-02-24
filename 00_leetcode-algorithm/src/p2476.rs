use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::with_capacity(queries.len());
    queries.iter().for_each(|&q| {
        let mut t = [-1, -1];
        dfs(&root, q, &mut t);
        ans.push(vec![t[0], t[1]]);
    });
    ans
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, query: i32, ans: &mut [i32; 2]) {
    if let Some(node) = node {
        let val = node.borrow().val;
        if val == query {
            ans[0] = val;
            ans[1] = val;
        } else if val < query {
            ans[0] = val;
            dfs(&node.borrow().right, query, ans);
        } else {
            ans[1] = val;
            dfs(&node.borrow().left, query, ans);
        }
    }
}

pub fn closest_nodes2(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nodes = vec![];
    inorder(&root, &mut nodes);

    let mut ans = Vec::with_capacity(queries.len());
    queries.iter().for_each(|q| match nodes.binary_search(q) {
        Ok(_) => {
            ans.push(vec![*q, *q]);
        }
        Err(idx) => {
            if idx == 0 {
                ans.push(vec![-1, nodes[idx]]);
            } else if idx == nodes.len() {
                ans.push(vec![nodes[idx - 1], -1]);
            } else {
                ans.push(vec![nodes[idx - 1], nodes[idx]]);
            }
        }
    });

    ans
}

fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<i32>) {
    if let Some(node) = node {
        inorder(&node.borrow().left, nodes);
        nodes.push(node.borrow().val);
        inorder(&node.borrow().right, nodes);
    }
}
