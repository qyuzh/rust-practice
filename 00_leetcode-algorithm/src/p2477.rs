/// get answer in O(n)/O(n)
pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let mut g = vec![vec![]; roads.len() + 1];
    for road in roads.iter() {
        g[road[0] as usize].push(road[1] as usize);
        g[road[1] as usize].push(road[0] as usize);
    }
    dfs(&g, 0, 0, seats as i64).1
}

/// # Returns
/// (the number of people, the number of liters of fuel has been consumed)
fn dfs(g: &Vec<Vec<usize>>, node: usize, fa: usize, seats: i64) -> (i64, i64) {
    // error: [[0,1],[1,2]], g[0].len() == 1, but should not return
    // if g[node].len() == 1 {
    //     return (1, 0);
    // }

    let mut ans = (1, 0);
    for &nxt in g[node].iter() {
        if nxt != fa {
            let t = dfs(g, nxt, node, seats);
            ans.0 += t.0;
            ans.1 += t.1 + (t.0 / seats) + if t.0 % seats == 0 { 0 } else { 1 };
        }
    }
    ans
}
