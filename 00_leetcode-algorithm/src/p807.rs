/// runs in O(n*m)/O(n+m)
pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut max_rows = vec![0; m];
    let mut max_cols = vec![0; n];
    for (i, r) in grid.iter().enumerate() {
        max_rows[i] = *r.iter().max().unwrap();
    }
    for i in 0..n {
        for j in 0..m {
            max_cols[i] = max_cols[i].max(grid[j][i]);
        }
    }
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            let x = grid[i][j];
            ans += max_rows[i].min(max_cols[j]) - x;
        }
    }
    ans
}
