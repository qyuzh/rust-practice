use std::collections::HashSet;

/// adjacency list
/// runs in O(n(n + m))/O(n + m), where m = edges.len()
pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut r_g = vec![vec![]; n as usize];
    for e in edges.iter() {
        r_g[e[1] as usize].push(e[0] as usize);
    }

    let mut ans = vec![HashSet::new(); n as usize];
    let mut vis = vec![false; n as usize];
    for i in (0..n as usize) {
        dfs(i, &r_g, &mut vis, &mut ans);
    }

    let mut ans2 = vec![];
    for t in ans.into_iter() {
        let mut tt = t.into_iter().collect::<Vec<i32>>();
        tt.sort_unstable();
        ans2.push(tt);
    }

    ans2
}

fn dfs(node: usize, r_g: &Vec<Vec<usize>>, vis: &mut Vec<bool>, ans: &mut Vec<HashSet<i32>>) {
    if vis[node] {
        return;
    }

    let mut fa = HashSet::new();

    for &nxt in r_g[node].iter() {
        fa.insert(nxt as i32);
        dfs(nxt, r_g, vis, ans);
        fa.extend(ans[nxt].clone());
    }

    ans[node] = fa;
    vis[node] = true;
}

/// adjacency list
/// runs in O(n(n + m))/O(n + m), where m = edges.len()
pub fn get_ancestors2(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in edges.iter() {
        g[e[1] as usize].push(e[0] as usize);
    }

    let mut ans = vec![vec![]; n];
    let mut vis = vec![false; n];

    for i in (0..n) {
        vis.fill(false);
        dfs2(&g, i, &mut vis);
        vis[i] = false;
        for (j, &v) in vis.iter().enumerate() {
            if v {
                ans[i].push(j as i32);
            }
        }
    }

    ans
}

fn dfs2(g: &Vec<Vec<usize>>, node: usize, vis: &mut Vec<bool>) {
    vis[node] = true;
    for &nxt in g[node].iter() {
        if !vis[nxt] {
            dfs2(g, nxt, vis);
        }
    }
}
