use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug)]
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

/// Runs in O(n^2)/O(n^2)
pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    root.as_ref()?;
    let leaves = find_leaves(&root);
    let deepest_leaves = find_deepest_leaves(leaves);
    let paths = find_paths_for_leaves(deepest_leaves, &root);
    find_common_ancestor(paths.as_slice())
}

pub fn find_leaves(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<(Rc<RefCell<TreeNode>>, usize)> {
    fn find_leaves_inner(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        leaves: &mut Vec<(Rc<RefCell<TreeNode>>, usize)>,
    ) {
        if let Some(node) = root {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                leaves.push((node.clone(), depth));
            } else {
                let b = node.borrow();
                find_leaves_inner(&b.left, depth + 1, leaves);
                find_leaves_inner(&b.right, depth + 1, leaves);
            }
        }
    }

    let mut leaves = vec![];
    find_leaves_inner(root, 0, &mut leaves);
    leaves
}

fn find_deepest_leaves(
    mut leaves: Vec<(Rc<RefCell<TreeNode>>, usize)>,
) -> Vec<Rc<RefCell<TreeNode>>> {
    let max_depth = leaves.iter().map(|(_, depth)| *depth).max().unwrap_or(0);
    leaves.retain(|(_, depth)| *depth == max_depth);
    leaves.into_iter().map(|(node, _)| node).collect()
}

type Path = Vec<Rc<RefCell<TreeNode>>>;

fn find_paths_for_leaves(
    deepest_leafs: Vec<Rc<RefCell<TreeNode>>>,
    root: &Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Path> {
    fn find_paths_inner(
        deepest_leafs: &[Rc<RefCell<TreeNode>>],
        root: &Option<Rc<RefCell<TreeNode>>>,
        paths: &mut Vec<Path>,
        path: &mut Path,
    ) {
        if let Some(node) = root {
            path.push(node.clone());
            if deepest_leafs.iter().any(|leaf| Rc::ptr_eq(node, leaf)) {
                paths.push(path.clone());
            } else {
                let b = node.borrow();
                find_paths_inner(deepest_leafs, &b.left, paths, path);
                find_paths_inner(deepest_leafs, &b.right, paths, path);
            }
            path.pop();
        }
    }

    let mut paths = vec![];
    let mut path = vec![];
    find_paths_inner(&deepest_leafs, root, &mut paths, &mut path);
    paths
}

fn find_common_ancestor(paths: &[Vec<Rc<RefCell<TreeNode>>>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut index = 0;

    while paths
        .iter()
        .all(|path| path.len() > index && Rc::ptr_eq(&path[index], &paths[0][index]))
    {
        index += 1;
    }

    if index == 0 {
        None
    } else {
        Some(paths[0][index - 1].clone())
    }
}

/// Runs in O(n)/O(n)
pub fn lca_deepest_leaves2(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut ans = None;
    let mut max_depth = -1; // 全局最大深度

    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        max_depth: &mut i32,
        ans: &mut Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        if let Some(node) = node {
            let x = node.borrow();
            let left_max_depth = dfs(&x.left, depth + 1, max_depth, ans); // 左子树最深空节点的深度
            let right_max_depth = dfs(&x.right, depth + 1, max_depth, ans); // 右子树最深空节点的深度
            if left_max_depth == right_max_depth && left_max_depth == *max_depth {
                // 最深的空节点左右子树都有
                *ans = Some(node.clone());
            }
            left_max_depth.max(right_max_depth) // 当前子树最深空节点的深度
        } else {
            *max_depth = (*max_depth).max(depth); // 维护全局最大深度
            depth
        }
    }

    dfs(&root, 0, &mut max_depth, &mut ans);
    ans
}

pub fn lca_deepest_leaves3(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    /// # Returns
    /// - 0, the depth of the deepest leaf node of the tree with node as the root
    /// - 1, the lca of the deepest leaf nodes of the tree with node as the root
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(cur) = node {
            let left = dfs(&cur.borrow().left);
            let right = dfs(&cur.borrow().right);
            match left.0.cmp(&right.0) {
                std::cmp::Ordering::Equal => (left.0 + 1, Some(cur.clone())),
                std::cmp::Ordering::Greater => (left.0 + 1, left.1),
                std::cmp::Ordering::Less => (right.0 + 1, right.1),
            }
        } else {
            (0, None)
        }
    }
    dfs(&root).1
}
