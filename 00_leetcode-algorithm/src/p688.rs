static DIRS: [(i32, i32); 8] = [
    (-2, -1), // right-top
    (-2, 1),  // right-bottom
    (-1, -2), // top-right
    (-1, 2),  // top-left
    (1, -2),  // bottom-right
    (1, 2),   // bottom-left
    (2, -1),  // left-top
    (2, 1),   // left-bottom
];

pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    /// A mutable HashMap to store memoized results for dynamic programming.
    /// This is used to avoid redundant calculations and improve performance.
    let mut memo = std::collections::HashMap::new();
    dfs(n, k, row, column, &mut memo)
}

fn dfs(
    n: i32,
    k: i32,
    row: i32,
    column: i32,
    memo: &mut std::collections::HashMap<(i32, i32, i32), f64>,
) -> f64 {
    if row < 0 || row >= n || column < 0 || column >= n {
        return 0.0;
    }
    if k == 0 {
        return 1.0;
    }
    if let Some(&res) = memo.get(&(k, row, column)) {
        return res;
    }
    let mut res = 0.0;
    for (dx, dy) in &DIRS {
        res += dfs(n, k - 1, row + dx, column + dy, memo);
    }
    let res = res / 8.0;
    memo.insert((k, row, column), res);
    res
}
