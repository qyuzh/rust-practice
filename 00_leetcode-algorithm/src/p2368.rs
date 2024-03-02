use std::collections::HashSet;

pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
    let tree = construct_tree(n as usize, edges);
    let restricted: HashSet<usize> = restricted.into_iter().map(|v| v as usize).collect();

    dfs(0, 0, &tree, &restricted) as i32
}

fn construct_tree(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut tree = vec![vec![]; n];

    edges.iter().for_each(|edge| {
        tree[edge[0] as usize].push(edge[1] as usize);
        tree[edge[1] as usize].push(edge[0] as usize);
    });

    tree
}

fn dfs(start: usize, fa: usize, tree: &Vec<Vec<usize>>, restricted: &HashSet<usize>) -> usize {
    if start >= tree.len() || restricted.contains(&start) {
        return 0;
    }

    let mut cnt = 1;
    tree[start].iter().for_each(|&nxt| {
        if nxt == fa {
            return;
        }
        cnt += dfs(nxt, start, tree, restricted);
    });

    cnt
}
