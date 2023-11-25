use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut feature = [0; 10];
    dfs(&root, &mut feature, 1) as i32
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, feature: &mut [usize], depth: usize) -> usize {
    if node.is_none() {
        return 0;
    }

    let node = node.as_ref().unwrap().borrow();
    let v = node.val;
    feature[v as usize] += 1;

    let cnt = if node.left == None && node.right == None {
        let mut odd_cnt = 0;
        for &x in feature.iter() {
            odd_cnt += x % 2;
        }
        if depth % 2 == 1 {
            if odd_cnt == 1 {
                1
            } else {
                0
            }
        } else {
            if odd_cnt == 0 {
                1
            } else {
                0
            }
        }
    } else {
        let cnt = dfs(&node.left, feature, depth + 1) + dfs(&node.right, feature, depth + 1);
        cnt
    };

    feature[v as usize] -= 1;
    cnt
}
