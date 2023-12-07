// get answer in O(n)/O(n)
pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut g = vec![vec![]; n as usize];
    for c in connections.iter() {
        g[c[0] as usize].push((c[1] as usize, true));
        g[c[1] as usize].push((c[0] as usize, false));
    }
    dfs(0, 0, &g)
}

fn dfs(node: usize, fa: usize, g: &Vec<Vec<(usize, bool)>>) -> i32 {
    let mut ans = 0;
    for &(nxt, to) in g[node].iter() {
        if nxt != fa {
            ans += dfs(nxt, node, g) + if to { 1 } else { 0 };
        }
    }
    ans
}
